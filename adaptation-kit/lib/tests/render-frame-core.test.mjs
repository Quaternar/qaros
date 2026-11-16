import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';
const source = readFileSync(new URL('../render-frame-core@1.js', import.meta.url), 'utf8');
function harness() {
 const queue = [], draws = [], acknowledgements = [];
 const window = { qaros: { completeRenderFrame: id => acknowledgements.push(id), failRenderFrame: (_, error) => { throw error; } } };
 vm.runInNewContext(source, { window, console, requestAnimationFrame: callback => queue.push(callback) });
 const loop = window.__qarAdaptationCore.createRenderFrameLoop(detail => { draws.push(detail.requestId); return true; });
 return { loop, draws, acknowledgements, tick: timestamp => queue.splice(0).forEach(callback => callback(timestamp)) };
}
test('continuous rendering produces a frame on every tick without another request', () => {
 const h = harness(); h.loop.update({ requestId: 1, continuous: true });
 h.tick(); h.tick(); h.tick();
 assert.deepEqual(h.draws, [1, 1, 1]); assert.deepEqual(h.acknowledgements, [1]);
});
test('updates coalesce to the newest pose and stop cancels queued work', () => {
 const h = harness(); h.loop.update({ requestId: 1, continuous: true }); h.loop.update({ requestId: 2, continuous: true });
 h.tick(); h.loop.update({ requestId: 3, continuous: true }); h.tick(); h.loop.stop(); h.tick();
 assert.deepEqual(h.draws, [2, 3]); assert.deepEqual(h.acknowledgements, [2, 3]);
});
test('legacy requests render once', () => {
 const h = harness(); h.loop.update({ requestId: 7 }); h.tick(); h.tick(); assert.deepEqual(h.draws, [7]);
});

test('pause stops rendering and the next request resumes it', () => {
 const h = harness(); h.loop.update({ requestId: 1, continuous: true }); h.tick(); h.loop.pause(); h.tick();
 h.loop.update({ requestId: 2, continuous: true }); h.tick(); assert.deepEqual(h.draws, [1, 2]);
});

 test('RenderRate 60 at 120 Hz ticks renders 60 FPS and use the newest pose without queuing history', () => {
 const h = harness(); h.loop.update({ requestId: 1, continuous: true, renderRate: 60 });
 h.tick(0); h.loop.update({ requestId: 2, continuous: true, renderRate: 60 }); h.tick(1000/120);
 h.loop.update({ requestId: 3, continuous: true, renderRate: 60 }); h.tick(1000/60);
 assert.deepEqual(h.draws, [1, 3]); assert.deepEqual(h.acknowledgements, [1, 3]);
 });

test('RenderRate 60 with jittery 60 Hz ticks renders on every tick', () => {
 const h = harness(); h.loop.update({ requestId: 1, continuous: true, renderRate: 60 });
 [0, 16.4, 33.5, 49.9, 66.8].forEach(time => h.tick(time));
 assert.equal(h.draws.length, 5);
});

test('a tick during pending work keeps the loop scheduled', () => {
 const queue = [];
 const window = { qaros: { completeRenderFrame() {}, failRenderFrame(_, e) { throw e; } } };
 vm.runInNewContext(source, { window, console, requestAnimationFrame: cb => queue.push(cb) });
 const loop = window.__qarAdaptationCore.createRenderFrameLoop(() => new Promise(() => {}));
 loop.update({ requestId: 1, continuous: true });
 queue.splice(0).forEach(cb => cb(0));
 queue.splice(0).forEach(cb => cb(16.7));
 assert.equal(queue.length, 1, 'the busy tick asked for the next frame');
});

test('asynchronous snapshots never overlap and resume with the newest pose', async () => {
 const queue = [], draws = [], completed = [];
 let release;
 const window = {qaros:{completeRenderFrame:id=>completed.push(id),failRenderFrame:(_,e)=>{throw e}}};
 vm.runInNewContext(source,{window,console,requestAnimationFrame:cb=>queue.push(cb)});
 const loop = window.__qarAdaptationCore.createRenderFrameLoop(detail=>{
   draws.push(detail.requestId); return new Promise(resolve=>{release=()=>resolve(true)});
 });
 const tick = time=>queue.splice(0).forEach(cb=>cb(time));
 loop.update({requestId:1,continuous:true});tick(0);
 loop.update({requestId:2,continuous:true});tick(20);
 loop.update({requestId:3,continuous:true});tick(40);
 assert.deepEqual(draws,[1]);assert.deepEqual(completed,[]);
 release();await new Promise(resolve=>setImmediate(resolve));tick(60);
 assert.deepEqual(draws,[1,3]);assert.deepEqual(completed,[1]);
 release();await new Promise(resolve=>setImmediate(resolve));loop.stop();tick(80);
 assert.deepEqual(completed,[1,3]);assert.deepEqual(draws,[1,3]);
});

// Two render-frame libraries in one profile (three.js WebGL and WebGPU): each request reaches the
// first handler that accepts it, never both.
function selectorHarness() {
	const listeners = {};
	const window = {
		qaros: { completeRenderFrame() {}, failRenderFrame(_, error) { throw error; } },
		addEventListener(name, listener) { (listeners[name] ||= []).push(listener); },
	};
	const context = { window, console, requestAnimationFrame() {} };
	vm.runInNewContext(source, context);
	const handler = (name, accepts) => {
		const calls = { updates: [], pauses: 0, stops: 0 };
		const loop = { update: (detail) => calls.updates.push(detail.requestId), pause: () => ++calls.pauses, stop: () => ++calls.stops };
		window.__qarAdaptationCore.addRenderFrameHandler({ name, accepts, loop });
		return calls;
	};
	const emit = (name, detail) => (listeners[name] || []).forEach((listener) => listener({ detail }));
	return { window, listeners, handler, emit, reload: () => vm.runInNewContext(source, context) };
}

test('each request goes to the one handler that accepts it', () => {
	const h = selectorHarness();
	let webgpu = false;
	const gl = h.handler('threejs-webgl', () => !webgpu);
	const gpu = h.handler('threejs-webgpu', () => webgpu);
	h.emit('qaros-render-frame', { requestId: 1 });
	webgpu = true;
	h.emit('qaros-render-frame', { requestId: 2 });
	assert.deepEqual(gl.updates, [1]);
	assert.deepEqual(gpu.updates, [2]);
	assert.equal(gl.pauses, 1, 'the handler that stopped answering is paused');
	h.emit('qaros-render-stop', {});
	assert.equal(gpu.pauses, 1);
	assert.equal(h.listeners['qaros-render-frame'].length, 1);
});

test('without an accepting handler the first one answers with its own error', () => {
	const h = selectorHarness();
	const first = h.handler('first', () => false);
	const second = h.handler('second', () => { throw new Error('not ready'); });
	h.emit('qaros-render-frame', { requestId: 5 });
	assert.deepEqual(first.updates, [5]);
	assert.deepEqual(second.updates, []);
});

test('a handler injected again replaces its registration and the core listens once', () => {
	const h = selectorHarness();
	const old = h.handler('threejs-webgl', () => true);
	h.reload();
	const replacement = h.handler('threejs-webgl', () => true);
	h.emit('qaros-render-frame', { requestId: 9 });
	assert.equal(old.stops, 1);
	assert.deepEqual(old.updates, []);
	assert.deepEqual(replacement.updates, [9]);
	assert.equal(h.listeners['qaros-render-frame'].length, 1);
});

// The profile's RenderRate caps the continuous loop: 1 s of 60 Hz ticks. Without one, 30 FPS.
function drawsOverOneSecond(detail, jitter = 0, hertz = 60) {
	const h = harness();
	h.loop.update(detail);
	for (let tick = 0; tick <= hertz; ++tick) h.tick(tick * (1000 / hertz) + (tick % 2 ? -jitter : jitter));
	return h.draws.length;
}

for (const rate of [15, 30, 60]) {
	test(`renderRate ${rate} draws about ${rate} times a second at 60 Hz ticks`, () => {
		const draws = drawsOverOneSecond({ requestId: 1, continuous: true, renderRate: rate });
		assert.ok(Math.abs(draws - rate) <= 1, `${draws} draws`);
	});
}

test('renderRate 30 with jittery 60 Hz ticks draws about 30 times a second', () => {
	const draws = drawsOverOneSecond({ requestId: 1, continuous: true, renderRate: 30 }, 1.5);
	assert.ok(Math.abs(draws - 30) <= 1, `${draws} draws`);
});

test('renderRate 60 at 120 Hz ticks draws about 60 times a second', () => {
	const draws = drawsOverOneSecond({ requestId: 1, continuous: true, renderRate: 60 }, 0, 120);
	assert.ok(Math.abs(draws - 60) <= 1, `${draws} draws`);
});

test('without renderRate the loop draws about 30 times a second', () => {
	for (const detail of [{ requestId: 1, continuous: true }, { requestId: 1, continuous: true, renderRate: 0 }]) {
		const draws = drawsOverOneSecond(detail);
		assert.ok(Math.abs(draws - 30) <= 1, `${draws} draws`);
	}
});

test('renderRate is clamped to 1..120', () => {
	const core = (() => { const window = {}; vm.runInNewContext(source, { window, console, requestAnimationFrame() {} }); return window.__qarAdaptationCore; })();
	assert.equal(core.renderRateOf({ renderRate: 500 }), 120);
	assert.equal(core.renderRateOf({ renderRate: 0.2 }), 1);
	assert.equal(core.renderRateOf({}), 30);
});
