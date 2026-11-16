// Shader compilation with explicit status checks -- a lesson this application's own briefing called
// out by name: a GLSL compile error is reported by WebGL only through `gl.getShaderInfoLog`, never
// thrown as a JS exception (three.js, for one, only ever logs it to the console). A shader that
// silently fails to compile still links into "a program" that draws nothing, or draws a uniform
// fallback colour -- exactly the "looks right and is wrong" failure mode a maths bug would also
// produce. Every shader and program this application (and its adaptation) builds goes through these
// two functions rather than calling `gl.compileShader`/`gl.linkProgram` directly, so a broken shader
// fails loudly with the offending source and info log instead of surfacing as unexplained black or
// blank output.

export function compileShader(gl, type, source) {
	const shader = gl.createShader(type);
	gl.shaderSource(shader, source);
	gl.compileShader(shader);
	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		const log = gl.getShaderInfoLog(shader);
		gl.deleteShader(shader);
		const kind = type === gl.VERTEX_SHADER ? "vertex" : "fragment";
		throw new Error(`WebGL ${kind} shader failed to compile: ${log}\n--- source ---\n${source}`);
	}
	return shader;
}

export function createProgram(gl, vertexSource, fragmentSource) {
	const vertexShader = compileShader(gl, gl.VERTEX_SHADER, vertexSource);
	const fragmentShader = compileShader(gl, gl.FRAGMENT_SHADER, fragmentSource);
	const program = gl.createProgram();
	gl.attachShader(program, vertexShader);
	gl.attachShader(program, fragmentShader);
	gl.linkProgram(program);
	// Shaders are flagged for deletion once detached; they stay alive until the program using them
	// is itself deleted, but there is no reason to keep the standalone shader objects around after a
	// successful link.
	gl.deleteShader(vertexShader);
	gl.deleteShader(fragmentShader);
	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		const log = gl.getProgramInfoLog(program);
		gl.deleteProgram(program);
		throw new Error(`WebGL program failed to link: ${log}`);
	}
	return program;
}

export function createBuffer(gl, target, data, usage = gl.STATIC_DRAW) {
	const buffer = gl.createBuffer();
	gl.bindBuffer(target, buffer);
	gl.bufferData(target, data, usage);
	return buffer;
}
