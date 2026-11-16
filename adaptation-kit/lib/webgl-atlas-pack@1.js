// @qaros-library webgl-atlas-pack@1.0
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
//   const pass = packer.ensurePass(gl, width, height); // { framebuffer, color, depth, width, height }
//   // ... render every eye into pass.framebuffer, eye.camera.viewport = { x, y, z: width, w: height }
//   // (y counted from the bottom of the pass) ...
//   packer.copyEyesToAtlas(gl, pass, detail.atlas.height, eyes);
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
		}`;

	// One texel per destination pixel: `source` + (pixel - `destination`). Opaque: Chromium
	// composites alpha < 1 premultiplied (atlas page) or over white (stream surface), which
	// changes the colour bytes, so the stream never depends on the app's canvas alpha.
	//
	// With a depth view, an empty texel (depth at the clear value) in a block that holds geometry
	// takes the colour of the block's nearest texel. Its packed depth and alpha are that texel's, so
	// without this it would reach users as the clear colour at the geometry's depth and opaque: a
	// dark fringe beside every edge. With it, colour, depth and alpha describe the same block.
	const copyFragment = `#version 300 es
		precision highp float;
		uniform highp sampler2D colorTexture;
		uniform highp sampler2D depthTexture;
		uniform ivec2 sourceOrigin;
		uniform ivec2 destinationOrigin;
		uniform ivec2 eyeSize;
		uniform int fillEmptyFromBlock;
		out vec4 outColor;
		void main() {
			ivec2 local = ivec2(gl_FragCoord.xy) - destinationOrigin;
			vec3 color = texelFetch(colorTexture, sourceOrigin + local, 0).rgb;
			if (fillEmptyFromBlock != 0 && texelFetch(depthTexture, sourceOrigin + local, 0).r >= 1.0) {
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
		uniform ivec2 eyeOrigin;
		uniform ivec2 eyeSize;
		uniform ivec2 destinationOrigin;
		uniform int halfWidth;
		uniform int tileHeight;
		out vec4 outColor;
		float eyeDepth(int x, int y) {
			return texelFetch(depthTexture, eyeOrigin + ivec2(x, eyeSize.y - 1 - y), 0).r;
		}
		float eyeAlpha(int x, int y) {
			return 0.0;
		}
		${blockFunctions}
		void main() {
			ivec2 local = ivec2(gl_FragCoord.xy) - destinationOrigin;
			int packedY = tileHeight - 1 - local.y;
			bool rightTile = local.x >= halfWidth;
			int column = rightTile ? local.x - halfWidth : local.x;
			int row = rightTile ? packedY + tileHeight : packedY;
			float rawDepth = blockDepth(column, row);
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

		function ensurePass(gl, width, height) {
			if (pass && pass.width === width && pass.height === height) {
				return pass;
			}
			if (pass) {
				gl.deleteFramebuffer(pass.framebuffer);
				gl.deleteTexture(pass.color);
				gl.deleteTexture(pass.depth);
			}
			const color = createTexture(gl, gl.RGBA8, width, height);
			const depth = createTexture(gl, gl.DEPTH_COMPONENT24, width, height);
			const framebuffer = gl.createFramebuffer();
			gl.bindFramebuffer(gl.FRAMEBUFFER, framebuffer);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, color, 0);
			gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.DEPTH_ATTACHMENT, gl.TEXTURE_2D, depth, 0);
			const status = gl.checkFramebufferStatus(gl.FRAMEBUFFER);
			gl.bindFramebuffer(gl.FRAMEBUFFER, null);
			if (status !== gl.FRAMEBUFFER_COMPLETE) {
				throw new Error(`QAR adaptation: eye framebuffer incomplete (0x${status.toString(16)})`);
			}
			pass = { width, height, color, depth, framebuffer };
			return pass;
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
				]),
				pack: compileProgram(gl, packFragment, [
					"depthTexture",
					"eyeOrigin",
					"eyeSize",
					"destinationOrigin",
					"halfWidth",
					"tileHeight",
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
					drawRect(gl, atlasHeight, rect);
				}
				if (eye.depth) {
					const rect = eye.depth.rect;
					gl.useProgram(pack.program);
					gl.bindTexture(gl.TEXTURE_2D, target.depth);
					gl.uniform1i(pack.uniforms.depthTexture, 0);
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

		return Object.freeze({ ensurePass, copyEyesToAtlas });
	}

	window.__qarWebglAtlasPack = Object.freeze({ createAtlasPacker });
})();
