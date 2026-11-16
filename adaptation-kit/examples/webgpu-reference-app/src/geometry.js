// Procedural geometry -- every shape this application draws is generated here from plain arithmetic,
// not loaded from a model file or a mesh/geometry library, keeping the "hand-written WebGPU, no
// framework" property honest all the way down to the vertex data. Identical generation logic to
// ../../webgl-reference-app/src/geometry.js -- pure vertex arithmetic, engine-agnostic, and not part
// of any adaptation surface, so duplicating it per reference application costs nothing and keeps each
// application self-contained.
//
// Every generator duplicates vertices per face rather than sharing them, so each face gets its own
// flat per-vertex normal (no vertex is shared between two differently-oriented faces) -- simple to
// reason about here, at the cost of some redundant vertices this application's scene is far too small
// to care about.

function normalize(v) {
	const length = Math.hypot(v[0], v[1], v[2]) || 1;
	return [v[0] / length, v[1] / length, v[2] / length];
}

function finalize(positions, normals, indices) {
	return {
		positions: new Float32Array(positions),
		normals: new Float32Array(normals),
		indices: new Uint16Array(indices),
	};
}

function pushTriangle(positions, normals, indices, a, b, c, normal) {
	const base = positions.length / 3;
	positions.push(...a, ...b, ...c);
	for (let i = 0; i < 3; i++) {
		normals.push(...normal);
	}
	indices.push(base, base + 1, base + 2);
}

function pushQuad(positions, normals, indices, a, b, c, d, normal) {
	const base = positions.length / 3;
	positions.push(...a, ...b, ...c, ...d);
	for (let i = 0; i < 4; i++) {
		normals.push(...normal);
	}
	indices.push(base, base + 1, base + 2, base, base + 2, base + 3);
}

export function createBox(size) {
	const h = size / 2;
	const positions = [];
	const normals = [];
	const indices = [];
	pushQuad(positions, normals, indices, [-h, -h, h], [h, -h, h], [h, h, h], [-h, h, h], [0, 0, 1]);
	pushQuad(positions, normals, indices, [h, -h, -h], [-h, -h, -h], [-h, h, -h], [h, h, -h], [0, 0, -1]);
	pushQuad(positions, normals, indices, [-h, -h, -h], [-h, -h, h], [-h, h, h], [-h, h, -h], [-1, 0, 0]);
	pushQuad(positions, normals, indices, [h, -h, h], [h, -h, -h], [h, h, -h], [h, h, h], [1, 0, 0]);
	pushQuad(positions, normals, indices, [-h, h, h], [h, h, h], [h, h, -h], [-h, h, -h], [0, 1, 0]);
	pushQuad(positions, normals, indices, [-h, -h, -h], [h, -h, -h], [h, -h, h], [-h, -h, h], [0, -1, 0]);
	return finalize(positions, normals, indices);
}

/// Six vertices (±X, ±Y, ±Z), eight faces -- one per octant. `signs.length ** 3` faces are built by
/// connecting the three axis vertices matching each octant's sign combination; the vertex order is
/// swapped for octants with an odd number of negative signs to keep every face's winding consistent
/// (immaterial for lighting here since face culling is never enabled, but kept correct on principle).
export function createOctahedron(radius) {
	const positions = [];
	const normals = [];
	const indices = [];
	const signs = [1, -1];
	for (const sx of signs) {
		for (const sy of signs) {
			for (const sz of signs) {
				let a = [sx * radius, 0, 0];
				let b = [0, sy * radius, 0];
				let c = [0, 0, sz * radius];
				if (sx * sy * sz < 0) {
					[b, c] = [c, b];
				}
				pushTriangle(positions, normals, indices, a, b, c, normalize([sx, sy, sz]));
			}
		}
	}
	return finalize(positions, normals, indices);
}

/// The standard 12-vertex/20-face icosahedron construction (three mutually orthogonal golden
/// rectangles). Each vertex direction from the origin is already its own outward normal for a shape
/// centred there, so a face's flat normal is simply its three vertices' averaged (renormalized)
/// direction -- winding-independent, so no face-culling risk either.
export function createIcosahedron(radius) {
	const t = (1 + Math.sqrt(5)) / 2;
	const rawVertices = [
		[-1, t, 0], [1, t, 0], [-1, -t, 0], [1, -t, 0],
		[0, -1, t], [0, 1, t], [0, -1, -t], [0, 1, -t],
		[t, 0, -1], [t, 0, 1], [-t, 0, -1], [-t, 0, 1],
	].map((v) => {
		const unit = normalize(v);
		return [unit[0] * radius, unit[1] * radius, unit[2] * radius];
	});
	const faces = [
		[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11],
		[1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8],
		[3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
		[4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1],
	];
	const positions = [];
	const normals = [];
	const indices = [];
	for (const [ia, ib, ic] of faces) {
		const a = rawVertices[ia];
		const b = rawVertices[ib];
		const c = rawVertices[ic];
		const normal = normalize([a[0] + b[0] + c[0], a[1] + b[1] + c[1], a[2] + b[2] + c[2]]);
		pushTriangle(positions, normals, indices, a, b, c, normal);
	}
	return finalize(positions, normals, indices);
}

/// A radial fan for the side (per-segment normal pointing outward and slightly up, matching the
/// cone's own slope) plus a flat bottom cap.
export function createCone(radius, height, segments) {
	const positions = [];
	const normals = [];
	const indices = [];
	const apex = [0, height / 2, 0];
	const baseY = -height / 2;
	const slope = radius / height;
	for (let i = 0; i < segments; i++) {
		const a0 = (i / segments) * Math.PI * 2;
		const a1 = ((i + 1) / segments) * Math.PI * 2;
		const p0 = [Math.cos(a0) * radius, baseY, Math.sin(a0) * radius];
		const p1 = [Math.cos(a1) * radius, baseY, Math.sin(a1) * radius];
		const mid = (a0 + a1) / 2;
		const sideNormal = normalize([Math.cos(mid), slope, Math.sin(mid)]);
		pushTriangle(positions, normals, indices, apex, p0, p1, sideNormal);
		pushTriangle(positions, normals, indices, [0, baseY, 0], p1, p0, [0, -1, 0]);
	}
	return finalize(positions, normals, indices);
}

/// A flat radial fan, used for the ground plane. Winding is irrelevant (no face culling anywhere in
/// this application), so the normal is simply "up".
export function createDisc(radius, segments) {
	const positions = [];
	const normals = [];
	const indices = [];
	for (let i = 0; i < segments; i++) {
		const a0 = (i / segments) * Math.PI * 2;
		const a1 = ((i + 1) / segments) * Math.PI * 2;
		const p0 = [Math.cos(a0) * radius, 0, Math.sin(a0) * radius];
		const p1 = [Math.cos(a1) * radius, 0, Math.sin(a1) * radius];
		pushTriangle(positions, normals, indices, [0, 0, 0], p1, p0, [0, 1, 0]);
	}
	return finalize(positions, normals, indices);
}
