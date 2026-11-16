// @qaros-library webgl-atlas-pack@1.1
// @stage afterLoad
//
// Packs eye images rendered with WebGL2 into the render-frame atlas: colour per eye, depth at half
// resolution, and under `TransportAlpha` coverage alpha at half resolution. Engine-agnostic: a
// handler renders its eyes however it likes into the offscreen pass this library owns (an RGBA8
// colour texture and a 24-bit depth texture, standard depth cleared to 1), then calls
// `copyEyesToAtlas`. Used by threejs-render-frame@1 and by raw WebGL handlers (the WebGL Aquarium
// sample).
//
//   const packer = window.__qarWebglAtlasPack.createAtlasPacker();
//   const pass = packer.ensurePass(gl, width, height); // { framebuffer, color, depth, width, height, samples }
//   // ... render every eye into pass.framebuffer, eye.camera.viewport = { x, y, z: width, w: height }
//   // (y counted from the bottom of the pass) ...
//   packer.copyEyesToAtlas(gl, pass, detail.atlas.height, eyes);
//
// Multisampling (since 1.1): `ensurePass(gl, width, height, { samples: 4 })` renders the eyes into
// multisampled colour and depth renderbuffers (`pass.framebuffer`); `copyEyesToAtlas` resolves
// them first. The count is the largest the GPU supports up to the one asked for; `pass.samples`
// says which (1: no multisampling). Colour is the average of a texel's samples, so a line thinner
// than a texel keeps its share of the texel instead of showing in some texels and not in others
// (dotted lines, moire on dense grids). Depth is never averaged: the resolve keeps one sample per
// texel, the depth a pass without multisampling would have had, so a texel on an edge is either
// in front or behind, never between. The pass's alpha should then be coverage: clear it to 0
// (`clearAlphaForCoverage`) before drawing. A texel some samples cover but whose kept depth sample
// missed keeps its own resolved colour instead of its block's nearest colour (below).
//
// `eyes` entries: `{ color, depth, alpha }` (the request's views for that eye, any may be absent),
// `width`, `height` (the eye's size in the pass) and `camera.viewport` (its rectangle there). The
// packer draws into the default framebuffer (the atlas) and leaves the vertex array unbound.
//
// The atlas contract (see the library reference, "Atlas packing"):
// - colour: one eye texel per atlas pixel, opaque;
// - depth: raw post-projection depth as 24-bit unsigned normalized RGB, at half linear resolution,
//   each sample the nearest depth of its 2x2 block, in two side-by-side tiles of half the view's
//   width and a quarter of its height, saturated at the far plane;
// - alpha: three half-resolution rows per atlas row (R, G, B), each sample the largest coverage of
//   its 2x2 block;
// - an empty texel (depth at the clear value) in a block that holds geometry takes the colour of
//   the block's nearest texel, so colour, depth and alpha describe the same block.
(() => {
	function createTexture(gl, internalFormat, width, height) {
		const texture = gl.createTexture();
		gl.bindTexture(gl.TEXTURE_2D, texture);
		gl.texStorage2D(gl.TEXTURE_2D, 1, internalFormat, width, height);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
		return texture;
	}

	// The largest sample count up to `requested` that both pass formats support; 1 for none.
	function supportedSampleCount(gl, requested) {
		const wanted = Math.floor(Number(requested) || 0);
		if (wanted <= 1) return 1;
		const counts = (format) => Array.from(gl.getInternalformatParameter(gl.RENDERBUFFER, format, gl.SAMPLES) || []);
		const depthCounts = counts(gl.DEPTH_COMPONENT24);
		const usable = counts(gl.RGBA8).filter((count) => count > 1 && count <= wanted && depthCounts.includes(count));
		return usable.length > 0 ? Math.max(...usable) : 1;
	}

	// Sets the alpha of the bound framebuffer's colour to 0 and leaves its colour, the clear value
	// and the colour mask as they were: drawn after it, a multisampled pass's alpha is each texel's
	// coverage, while its edges still blend with the page's own clear colour.
	function clearAlphaForCoverage(gl) {
		const clearValue = gl.getParameter(gl.COLOR_CLEAR_VALUE);
		const mask = gl.getParameter(gl.COLOR_WRITEMASK);
		gl.colorMask(false, false, false, true);
		gl.clearColor(0, 0, 0, 0);
		gl.clear(gl.COLOR_BUFFER_BIT);
		gl.clearColor(clearValue[0], clearValue[1], clearValue[2], clearValue[3]);
		gl.colorMask(mask[0], mask[1], mask[2], mask[3]);
	}

	const fullScreenVertex = `#version 300 es
		void main() {
			vec2 corner = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2));
			gl_Position = vec4(corner * 2.0 - 1.0, 0.0, 1.0);
		}`;

	// Depth and alpha travel at half resolution: each sample stands for a 2x2 block of eye texels,
	// and native code nearest-upsamples it back over that block. A sample has to keep what any of
	// its four texels holds, not one texel of them: a feature thinner than the block (a 1-px line)
	// misses a fixed texel in most blocks, and its colour then reaches the mixer with no depth, so
	// the warp and the mixer drop it and the line shows dashed. Depth keeps the block's nearest
	// texel. The eye pass uses standard depth (cleared to 1, the far plane, tested LessEqual), so
	// the nearest is the smallest. Alpha keeps the block's largest coverage (a mean would dim a 1-px
	// line to a quarter or a half). `blockDepth` and `blockAlpha` take eye texels counted from the
	// eye's top row, and stay scalar so tests/threejs-depth-pack.test.mjs can run them as
	// JavaScript.
	//
	// Multisampled, a line thinner than a texel covers texels whose kept depth sample it missed: its
	// colour is there (alpha above 0) but no texel of the block has depth, and the mixer would drop
	// the block, so the line shows dashed again. `coveredBlockDepth` gives such a block the nearest
	// depth of the 8x8 texels around it, the depth of the geometry that covers it. It is still one
	// texel's depth, never a blend.
	const blockFunctions = `
		float blockDepth(int column, int row) {
			float nearest = 1.0;
			for (int dy = 0; dy < 2; ++dy) {
				for (int dx = 0; dx < 2; ++dx) {
					int x = min(column * 2 + dx, eyeSize.x - 1);
					int y = min(row * 2 + dy, eyeSize.y - 1);
					nearest = min(nearest, eyeDepth(x, y));
				}
			}
			return nearest;
		}
		float blockAlpha(int column, int row) {
			if (row * 2 >= eyeSize.y) return 0.0;
			float coverage = 0.0;
			for (int dy = 0; dy < 2; ++dy) {
				for (int dx = 0; dx < 2; ++dx) {
					int x = min(column * 2 + dx, eyeSize.x - 1);
					int y = min(row * 2 + dy, eyeSize.y - 1);
					coverage = max(coverage, eyeAlpha(x, y));
				}
			}
			return coverage;
		}
		float coveredBlockDepth(int column, int row) {
			float nearest = blockDepth(column, row);
			if (nearest < 1.0 || blockAlpha(column, row) < 0.5 / 255.0) return nearest;
			for (int dy = -3; dy < 5; ++dy) {
				for (int dx = -3; dx < 5; ++dx) {
					int x = max(min(column * 2 + dx, eyeSize.x - 1), 0);
					int y = max(min(row * 2 + dy, eyeSize.y - 1), 0);
					nearest = min(nearest, eyeDepth(x, y));
				}
			}
			return nearest;
		}`;

	// One texel per destination pixel: `source` + (pixel - `destination`). Opaque: Chromium
	// composites alpha < 1 premultiplied (atlas page) or over white (stream surface), which
	// changes the colour bytes, so the stream never depends on the app's canvas alpha.
	//
	// With a depth view, an empty texel (depth at the clear value) in a block that holds geometry
	// takes the colour of the block's nearest texel. Its packed depth and alpha are that texel's, so
	// without this it would reach users as the clear colour at the geometry's depth and opaque: a
	// dark fringe beside every edge. With it, colour, depth and alpha describe the same block.
	// Multisampled, an empty texel that some samples cover (alpha above 0, the pass cleared to alpha
	// 0) already holds its share of the geometry's colour and keeps it: replacing it with the block's
	// nearest colour would widen every antialiased line to whole blocks.
	const copyFragment = `#version 300 es
		precision highp float;
		uniform highp sampler2D colorTexture;
		uniform highp sampler2D depthTexture;
		uniform ivec2 sourceOrigin;
		uniform ivec2 destinationOrigin;
		uniform ivec2 eyeSize;
		uniform int fillEmptyFromBlock;
		uniform int keepCoveredColor;
		out vec4 outColor;
		void main() {
			ivec2 local = ivec2(gl_FragCoord.xy) - destinationOrigin;
			vec4 own = texelFetch(colorTexture, sourceOrigin + local, 0);
			vec3 color = own.rgb;
			bool covered = keepCoveredColor != 0 && own.a > 0.5 / 255.0;
			if (fillEmptyFromBlock != 0 && !covered && texelFetch(depthTexture, sourceOrigin + local, 0).r >= 1.0) {
				int left = local.x - local.x % 2;
				int top = eyeSize.y - 1 - local.y;
				top -= top % 2;
				float nearest = 1.0;
				for (int dy = 0; dy < 2; ++dy) {
					for (int dx = 0; dx < 2; ++dx) {
						ivec2 texel = sourceOrigin
							+ ivec2(min(left + dx, eyeSize.x - 1), eyeSize.y - 1 - min(top + dy, eyeSize.y - 1));
						float depth = texelFetch(depthTexture, texel, 0).r;
						if (depth < nearest) {
							nearest = depth;
							color = texelFetch(colorTexture, texel, 0).rgb;
						}
					}
				}
			}
			outColor = vec4(color, 1.0);
		}`;

	// Row 0 of the packed rectangle is the view's top row, the same atlas convention every colour
	// view relies on. Each half-resolution sample is the nearest depth of its 2x2 block (above),
	// never a filtered blend.
	const packFragment = `#version 300 es
		precision highp float;
		uniform highp sampler2D depthTexture;
		uniform highp sampler2D colorTexture;
		uniform ivec2 eyeOrigin;
		uniform ivec2 eyeSize;
		uniform ivec2 destinationOrigin;
		uniform int halfWidth;
		uniform int tileHeight;
		uniform int coverageDepth;
		out vec4 outColor;
		float eyeDepth(int x, int y) {
			return texelFetch(depthTexture, eyeOrigin + ivec2(x, eyeSize.y - 1 - y), 0).r;
		}
		float eyeAlpha(int x, int y) {
			return texelFetch(colorTexture, eyeOrigin + ivec2(x, eyeSize.y - 1 - y), 0).a;
		}
		${blockFunctions}
		void main() {
			ivec2 local = ivec2(gl_FragCoord.xy) - destinationOrigin;
			int packedY = tileHeight - 1 - local.y;
			bool rightTile = local.x >= halfWidth;
			int column = rightTile ? local.x - halfWidth : local.x;
			int row = rightTile ? packedY + tileHeight : packedY;
			float rawDepth = blockDepth(column, row);
			if (coverageDepth != 0) rawDepth = coveredBlockDepth(column, row);
			// At depth 1, float32 rounds 16777215 + 0.5 to 16777216. Saturate before
			// splitting bytes, otherwise the far plane wraps to near-plane black.
			float scaled = min(floor(clamp(rawDepth, 0.0, 1.0) * 16777215.0 + 0.5), 16777215.0);
			outColor = vec4(
				mod(scaled, 256.0) / 255.0,
				mod(floor(scaled / 256.0), 256.0) / 255.0,
				mod(floor(scaled / 65536.0), 256.0) / 255.0,
				1.0
			);
		}`;

	// Alpha at half linear resolution, three consecutive half-resolution rows per atlas row (R, G,
	// B), each sample the largest coverage of its 2x2 block (above). The colour copy fills the
	// block's empty texels, so the larger coverage never shows the clear colour. Opaque, so CEF
	// keeps every byte; native unpacks it into the colour view's alpha channel.
	const alphaPackFragment = `#version 300 es
		precision highp float;
		uniform highp sampler2D colorTexture;
		uniform ivec2 eyeOrigin;
		uniform ivec2 eyeSize;
		uniform ivec2 destinationOrigin;
		uniform int tileHeight;
		out vec4 outColor;
		float eyeDepth(int x, int y) {
			return 1.0;
		}
		float eyeAlpha(int x, int y) {
			return texelFetch(colorTexture, eyeOrigin + ivec2(x, eyeSize.y - 1 - y), 0).a;
		}
		${blockFunctions}
		float halfAlpha(int column, int row) {
			return blockAlpha(column, row);
		}
		void main() {
			ivec2 local = ivec2(gl_FragCoord.xy) - destinationOrigin;
			int row = (tileHeight - 1 - local.y) * 3;
			outColor = vec4(halfAlpha(local.x, row), halfAlpha(local.x, row + 1), halfAlpha(local.x, row + 2), 1.0);
		}`;

	function compileProgram(gl, fragmentSource, uniformNames) {
		const shader = (type, source) => {
			const object = gl.createShader(type);
			gl.shaderSource(object, source);
			gl.compileShader(object);
			if (!gl.getShaderParameter(object, gl.COMPILE_STATUS)) {
				throw new Error("QAR adaptation: shader compile failed: " + gl.getShaderInfoLog(object));
			}
			return object;
		};
		const program = gl.createProgram();
		gl.attachShader(program, shader(gl.VERTEX_SHADER, fullScreenVertex));
		gl.attachShader(program, shader(gl.FRAGMENT_SHADER, fragmentSource));
		gl.linkProgram(program);
		if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
			throw new Error("QAR adaptation: shader link failed: " + gl.getProgramInfoLog(program));
		}
		const uniforms = {};
		for (const name of uniformNames) {
			uniforms[name] = gl.getUniformLocation(program, name);
		}
		return { program, uniforms };
	}

	function drawRect(gl, atlasHeight, rect) {
		const y = atlasHeight - rect.y - rect.height;
		gl.viewport(rect.x, y, rect.width, rect.height);
		gl.scissor(rect.x, y, rect.width, rect.height);
		gl.drawArrays(gl.TRIANGLES, 0, 3);
		return y;
	}

	/// One packer per WebGL context: it owns that context's offscreen pass and pack programs.
	function createAtlasPacker() {
		let pass = null;
		let programs = null;

		function deletePass(gl) {
			gl.deleteFramebuffer(pass.resolveFramebuffer);
			gl.deleteTexture(pass.color);
			gl.deleteTexture(pass.depth);
			if (pass.samples > 1) {
				gl.deleteFramebuffer(pass.framebuffer);
				gl.deleteRenderbuffer(pass.colorSamples);
				gl.deleteRenderbuffer(pass.depthSamples);
			}
			pass = null;
		}

		function completeFramebuffer(gl, framebuffer) {
			const status = gl.checkFramebufferStatus(gl.FRAMEBUFFER);
			gl.bindFramebuffer(gl.FRAMEBUFFER, null);
			if (status !== gl.FRAMEBUFFER_COMPLETE) {
				gl.deleteFramebuffer(framebuffer);
				throw new Error(`QAR adaptation: eye framebuffer incomplete (0x${status.toString(16)})`);
			}
		}

		function createSampleRenderbuffer(gl, samples, internalFormat, width, height) {
			const renderbuffer = gl.createRenderbuffer();
			gl.bindRenderbuffer(gl.RENDERBUFFER, renderbuffer);
			gl.renderbufferStorageMultisample(gl.RENDERBUFFER, samples, internalFormat, width, height);
			gl.bindRenderbuffer(gl.RENDERBUFFER, null);
			return renderbuffer;
		}

		/// `options.samples`: samples per texel of the eye pass, 0 or 1 for none (the default).
		function ensurePass(gl, width, height, options) {
			const samples = supportedSampleCount(gl, options && options.samples);
			if (pass && pass.width === width && pass.height === height && pass.samples === samples) {
				return pass;
			}
			if (pass) deletePass(gl);
			const color = createTexture(gl, gl.RGBA8, width, height);
			const depth = createTexture(gl, gl.DEPTH_COMPONENT24, width, height);
			const resolveFramebuffer = gl.createFramebuffer();
			gl.bindFramebuffer(gl.FRAMEBUFFER, resolveFramebuffer);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, color, 0);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, depth, 0);
			completeFramebuffer(gl, resolveFramebuffer);
			pass = { width, height, color, depth, samples, framebuffer: resolveFramebuffer, resolveFramebuffer };
			if (samples > 1) {
				// Same formats as the textures: a multisample blit resolves only between matching ones.
				pass.colorSamples = createSampleRenderbuffer(gl, samples, gl.RGBA8, width, height);
				pass.depthSamples = createSampleRenderbuffer(gl, samples, gl.DEPTH_COMPONENT24, width, height);
				pass.framebuffer = gl.createFramebuffer();
				gl.bindFramebuffer(gl.FRAMEBUFFER, pass.framebuffer);
				gl.framebufferRenderbuffer(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.RENDERBUFFER, pass.colorSamples);
				gl.framebufferRenderbuffer(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.RENDERBUFFER, pass.depthSamples);
				completeFramebuffer(gl, pass.framebuffer);
			}
			return pass;
		}

		// Colour averaged over each texel's samples; depth one sample per texel (a blit never
		// averages depth, and NEAREST is the only filter it allows for depth).
		function resolvePass(gl, target) {
			if (!(target.samples > 1)) return;
			gl.disable(gl.SCISSOR_TEST);
			gl.colorMask(true, true, true, true);
			gl.depthMask(true);
			gl.bindFramebuffer(gl.READ_FRAMEBUFFER, target.framebuffer);
			gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, target.resolveFramebuffer);
			gl.blitFramebuffer(
				0, 0, target.width, target.height,
				0, 0, target.width, target.height,
				gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT,
				gl.NEAREST
			);
			gl.bindFramebuffer(gl.READ_FRAMEBUFFER, null);
			gl.bindFramebuffer(gl.DRAW_FRAMEBUFFER, null);
		}

		function ensurePrograms(gl) {
			if (programs) {
				return programs;
			}
			programs = {
				copy: compileProgram(gl, copyFragment, [
					"colorTexture",
					"depthTexture",
					"sourceOrigin",
					"destinationOrigin",
					"eyeSize",
					"fillEmptyFromBlock",
					"keepCoveredColor",
				]),
				pack: compileProgram(gl, packFragment, [
					"depthTexture",
					"eyeOrigin",
					"eyeSize",
					"destinationOrigin",
					"halfWidth",
					"tileHeight",
					"colorTexture",
					"coverageDepth",
				]),
				alpha: compileProgram(gl, alphaPackFragment, [
					"colorTexture",
					"eyeOrigin",
					"eyeSize",
					"destinationOrigin",
					"tileHeight",
				]),
				vertexArray: gl.createVertexArray(),
			};
			return programs;
		}

		function copyEyesToAtlas(gl, target, atlasHeight, eyes) {
			const { copy, pack, alpha, vertexArray } = ensurePrograms(gl);
			resolvePass(gl, target);
			gl.bindFramebuffer(gl.FRAMEBUFFER, null);
			gl.bindVertexArray(vertexArray);
			gl.disable(gl.DEPTH_TEST);
			gl.disable(gl.BLEND);
			gl.disable(gl.CULL_FACE);
			gl.disable(gl.STENCIL_TEST);
			gl.enable(gl.SCISSOR_TEST);
			gl.colorMask(true, true, true, true);
			gl.activeTexture(gl.TEXTURE0);
			for (const eye of eyes) {
				const viewport = eye.camera.viewport;
				if (eye.color) {
					const rect = eye.color.rect;
					gl.useProgram(copy.program);
					gl.activeTexture(gl.TEXTURE1);
					gl.bindTexture(gl.TEXTURE_2D, target.depth);
					gl.activeTexture(gl.TEXTURE0);
					gl.bindTexture(gl.TEXTURE_2D, target.color);
					gl.uniform1i(copy.uniforms.colorTexture, 0);
					gl.uniform1i(copy.uniforms.depthTexture, 1);
					gl.uniform2i(copy.uniforms.sourceOrigin, viewport.x, viewport.y);
					gl.uniform2i(copy.uniforms.destinationOrigin, rect.x, atlasHeight - rect.y - rect.height);
					gl.uniform2i(copy.uniforms.eyeSize, eye.width, eye.height);
					gl.uniform1i(copy.uniforms.fillEmptyFromBlock, eye.depth ? 1 : 0);
					gl.uniform1i(copy.uniforms.keepCoveredColor, target.samples > 1 ? 1 : 0);
					drawRect(gl, atlasHeight, rect);
				}
				if (eye.depth) {
					const rect = eye.depth.rect;
					gl.useProgram(pack.program);
					gl.activeTexture(gl.TEXTURE1);
					gl.bindTexture(gl.TEXTURE_2D, target.color);
					gl.activeTexture(gl.TEXTURE0);
					gl.bindTexture(gl.TEXTURE_2D, target.depth);
					gl.uniform1i(pack.uniforms.depthTexture, 0);
					gl.uniform1i(pack.uniforms.colorTexture, 1);
					gl.uniform1i(pack.uniforms.coverageDepth, target.samples > 1 ? 1 : 0);
					gl.uniform2i(pack.uniforms.eyeOrigin, viewport.x, viewport.y);
					gl.uniform2i(pack.uniforms.eyeSize, eye.width, eye.height);
					gl.uniform2i(pack.uniforms.destinationOrigin, rect.x, atlasHeight - rect.y - rect.height);
					gl.uniform1i(pack.uniforms.halfWidth, eye.depth.logicalWidth / 2);
					gl.uniform1i(pack.uniforms.tileHeight, rect.height);
					drawRect(gl, atlasHeight, rect);
				}
				if (eye.alpha) {
					const rect = eye.alpha.rect;
					gl.useProgram(alpha.program);
					gl.bindTexture(gl.TEXTURE_2D, target.color);
					gl.uniform1i(alpha.uniforms.colorTexture, 0);
					gl.uniform2i(alpha.uniforms.eyeOrigin, viewport.x, viewport.y);
					gl.uniform2i(alpha.uniforms.eyeSize, eye.width, eye.height);
					gl.uniform2i(alpha.uniforms.destinationOrigin, rect.x, atlasHeight - rect.y - rect.height);
					gl.uniform1i(alpha.uniforms.tileHeight, rect.height);
					drawRect(gl, atlasHeight, rect);
				}
			}
			gl.bindVertexArray(null);
		}

		return Object.freeze({ ensurePass, copyEyesToAtlas, clearAlphaForCoverage });
	}

	window.__qarWebglAtlasPack = Object.freeze({ createAtlasPacker, supportedSampleCount });
})();
