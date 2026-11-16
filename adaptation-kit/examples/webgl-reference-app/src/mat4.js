// Minimal, hand-rolled column-major 4x4 matrix helpers -- this application imports no matrix
// library, however small, consistent with "hand-written WebGL2" and "exposes no framework global":
// every global this application exposes is its own application-specific one (see main.js's own doc
// comment), never a named third-party library the probe's `kProbeKnownEngines` list -- or an agent
// reading `Object.keys(window)` -- could recognise.
//
// Column-major, `Float32Array`-backed, matching WebGL's own uniform-upload convention
// (`m[column * 4 + row]`), the same layout `gl.uniformMatrix4fv` expects without transposing.

export function identity(out = new Float32Array(16)) {
	out.set([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
	return out;
}

/// Standard quaternion (x, y, z, w) to rotation matrix, assuming a right-handed, active rotation --
/// the same convention `RenderStreamProtocol.cpp`'s `PoseJson` emits (`{x, y, z, w}` order).
export function fromQuaternion(out, x, y, z, w) {
	const xx = x * x;
	const yy = y * y;
	const zz = z * z;
	const xy = x * y;
	const xz = x * z;
	const yz = y * z;
	const wx = w * x;
	const wy = w * y;
	const wz = w * z;
	out[0] = 1 - 2 * (yy + zz);
	out[1] = 2 * (xy + wz);
	out[2] = 2 * (xz - wy);
	out[3] = 0;
	out[4] = 2 * (xy - wz);
	out[5] = 1 - 2 * (xx + zz);
	out[6] = 2 * (yz + wx);
	out[7] = 0;
	out[8] = 2 * (xz + wy);
	out[9] = 2 * (yz - wx);
	out[10] = 1 - 2 * (xx + yy);
	out[11] = 0;
	out[12] = 0;
	out[13] = 0;
	out[14] = 0;
	out[15] = 1;
	return out;
}

/// Builds a view matrix (world-to-camera) directly from a camera's own world pose (position plus
/// the same camera-to-world rotation `fromQuaternion` produces), i.e. the inverse of a rigid
/// transform: `view = [Rᵀ | -Rᵀ·position]`. Written out directly rather than composing a general
/// 4x4 inverse, since a rotation-plus-translation matrix is always cheaply and exactly invertible
/// this way.
export function viewFromPose(out, position, orientation) {
	const rotation = fromQuaternion(new Float32Array(16), orientation[0], orientation[1], orientation[2], orientation[3]);
	// Transpose in place into `out` (a rotation matrix's inverse is its transpose).
	out[0] = rotation[0];
	out[1] = rotation[4];
	out[2] = rotation[8];
	out[4] = rotation[1];
	out[5] = rotation[5];
	out[6] = rotation[9];
	out[8] = rotation[2];
	out[9] = rotation[6];
	out[10] = rotation[10];
	out[3] = 0;
	out[7] = 0;
	out[11] = 0;
	out[15] = 1;
	const px = position[0];
	const py = position[1];
	const pz = position[2];
	out[12] = -(out[0] * px + out[4] * py + out[8] * pz);
	out[13] = -(out[1] * px + out[5] * py + out[9] * pz);
	out[14] = -(out[2] * px + out[6] * py + out[10] * pz);
	return out;
}

/// Builds an asymmetric OpenGL-style perspective projection directly from the four frustum angles
/// (`tan(angle) * near` gives each frustum-plane offset at the near plane), rather than a single
/// fov/aspect pair -- the two eyes of a stereo pair are not generally symmetric, and this is exactly
/// the shape `RenderStreamProtocol.cpp`'s `CameraJson` (and every other atlas-painting script in this
/// module) supplies per camera. NDC z lands in the standard OpenGL/WebGL [-1, 1] range, matching this
/// application's own default camera below and its raw hardware depth buffer -- see
/// the kit's `lib/webgl-render-frame-handler@1.js`'s own doc comment on why this application's depth convention
/// is always "regular".
export function perspectiveFromAngles(out, angleLeft, angleRight, angleUp, angleDown, near, far) {
	const left = Math.tan(angleLeft) * near;
	const right = Math.tan(angleRight) * near;
	const top = Math.tan(angleUp) * near;
	const bottom = Math.tan(angleDown) * near;
	const x = (2 * near) / (right - left);
	const y = (2 * near) / (top - bottom);
	const a = (right + left) / (right - left);
	const b = (top + bottom) / (top - bottom);
	const c = -(far + near) / (far - near);
	const d = -(2 * far * near) / (far - near);
	out.set([x, 0, 0, 0, 0, y, 0, 0, a, b, c, -1, 0, 0, d, 0]);
	return out;
}

/// The application's own default camera: an ordinary symmetric perspective, expressed as the same
/// four-angle frustum `perspectiveFromAngles` already takes, so the application and the adaptation
/// share one projection code path.
export function perspectiveFromFov(out, verticalFovRadians, aspect, near, far) {
	const halfHeightAngle = verticalFovRadians / 2;
	const halfWidthAngle = Math.atan(Math.tan(halfHeightAngle) * aspect);
	return perspectiveFromAngles(out, -halfWidthAngle, halfWidthAngle, halfHeightAngle, -halfHeightAngle, near, far);
}

/// An ordinary right-handed "look at" view matrix, used only for this application's own static
/// establishing camera -- unrelated to `viewFromPose`, which instead reconstructs a view matrix from
/// a pose (position plus quaternion) because that is the shape a target application's camera arrives
/// in from native code, not a look-at target.
export function lookAt(out, eye, target, up) {
	let zx = eye[0] - target[0];
	let zy = eye[1] - target[1];
	let zz = eye[2] - target[2];
	const zLength = Math.hypot(zx, zy, zz) || 1;
	zx /= zLength;
	zy /= zLength;
	zz /= zLength;
	let xx = up[1] * zz - up[2] * zy;
	let xy = up[2] * zx - up[0] * zz;
	let xz = up[0] * zy - up[1] * zx;
	const xLength = Math.hypot(xx, xy, xz) || 1;
	xx /= xLength;
	xy /= xLength;
	xz /= xLength;
	const yx = zy * xz - zz * xy;
	const yy = zz * xx - zx * xz;
	const yz = zx * xy - zy * xx;
	out[0] = xx;
	out[1] = yx;
	out[2] = zx;
	out[3] = 0;
	out[4] = xy;
	out[5] = yy;
	out[6] = zy;
	out[7] = 0;
	out[8] = xz;
	out[9] = yz;
	out[10] = zz;
	out[11] = 0;
	out[12] = -(xx * eye[0] + xy * eye[1] + xz * eye[2]);
	out[13] = -(yx * eye[0] + yy * eye[1] + yz * eye[2]);
	out[14] = -(zx * eye[0] + zy * eye[1] + zz * eye[2]);
	out[15] = 1;
	return out;
}

/// A rotation-about-Y-then-translate model matrix -- every object in this application's scene is
/// positioned and spun with nothing more elaborate than this.
export function modelFromPositionYaw(out, position, yaw) {
	const c = Math.cos(yaw);
	const s = Math.sin(yaw);
	out[0] = c;
	out[1] = 0;
	out[2] = -s;
	out[3] = 0;
	out[4] = 0;
	out[5] = 1;
	out[6] = 0;
	out[7] = 0;
	out[8] = s;
	out[9] = 0;
	out[10] = c;
	out[11] = 0;
	out[12] = position[0];
	out[13] = position[1];
	out[14] = position[2];
	out[15] = 1;
	return out;
}
