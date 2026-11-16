// Device setup and shader-module creation with explicit compilation checks.
//
// Unlike raw WebGL (see ../../webgl-reference-app/src/gl-utils.js's own doc comment: a GLSL compile
// error is only ever logged, never thrown), WebGPU's `GPUShaderModule.getCompilationInfo()` gives an
// explicit, awaitable list of compilation messages, and `device.createRenderPipelineAsync` rejects
// its returned promise on a pipeline that fails to validate. This application still checks
// `getCompilationInfo()` explicitly and throws on an "error"-severity message rather than relying
// only on a later pipeline-creation rejection, so a broken shader is reported with its own file/line
// rather than a comparatively opaque pipeline-validation failure several calls later.

export async function createCheckedShaderModule(device, code, label) {
	const module = device.createShaderModule({ label, code });
	const info = await module.getCompilationInfo();
	const errors = info.messages.filter((message) => message.type === "error");
	if (errors.length > 0) {
		const formatted = errors
			.map((message) => `  line ${message.lineNum}:${message.linePos}: ${message.message}`)
			.join("\n");
		throw new Error(`WebGPU shader module "${label}" failed to compile:\n${formatted}\n--- source ---\n${code}`);
	}
	return module;
}

/// Requests a high-performance adapter and device, then configures `canvas`'s `webgpu` context with
/// the platform's preferred swap-chain format. Throws with an actionable message at each of the three
/// steps that can fail (no adapter, no device, no context) rather than letting a `null` propagate
/// into a later, harder-to-diagnose call.
export async function initWebGpu(canvas) {
	if (!navigator.gpu) {
		throw new Error("WebGPU is not available (navigator.gpu is undefined).");
	}
	const adapter = await navigator.gpu.requestAdapter({ powerPreference: "high-performance" });
	if (!adapter) {
		throw new Error("WebGPU adapter request returned null.");
	}
	const device = await adapter.requestDevice();
	const context = canvas.getContext("webgpu");
	if (!context) {
		throw new Error("Failed to acquire a webgpu canvas context.");
	}
	const format = navigator.gpu.getPreferredCanvasFormat();
	context.configure({ device, format, alphaMode: "opaque" });
	return { adapter, device, context, format };
}

export function createUniformBuffer(device, size, label) {
	return device.createBuffer({
		label,
		size,
		usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
	});
}

export function createVertexBuffer(device, data, label) {
	const buffer = device.createBuffer({
		label,
		size: (data.byteLength + 3) & ~3,
		usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
		mappedAtCreation: true,
	});
	new (data.constructor)(buffer.getMappedRange()).set(data);
	buffer.unmap();
	return buffer;
}

export function createIndexBuffer(device, data, label) {
	const buffer = device.createBuffer({
		label,
		size: (data.byteLength + 3) & ~3,
		usage: GPUBufferUsage.INDEX | GPUBufferUsage.COPY_DST,
		mappedAtCreation: true,
	});
	new Uint16Array(buffer.getMappedRange()).set(data);
	buffer.unmap();
	return buffer;
}
