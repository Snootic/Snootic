import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import vm from 'node:vm';
import * as THREE from 'three';

const source = await readFile(new URL('../src/three/panorama.js', import.meta.url), 'utf8');
const flush = () => new Promise(resolve => setImmediate(resolve));

function setup({ reduced = false, hidden = false } = {}) {
  const events = () => ({
    listeners: new Map(),
    addEventListener(name, callback) { this.listeners.set(name, callback); },
    removeEventListener(name, callback) {
      if (this.listeners.get(name) === callback) this.listeners.delete(name);
    },
    fire(name) { this.listeners.get(name)?.(); },
  });
  const media = { ...events(), matches: reduced };
  const document = { ...events(), hidden };
  const window = {
    ...events(), innerWidth: 800, innerHeight: 600, devicePixelRatio: 1,
    matchMedia: () => media,
  };
  const renderers = [];
  const frames = new Map();
  let id = 0;
  let fail = false;
  class Renderer {
    constructor() {
      if (fail) throw new Error('WebGL unavailable');
      this.renders = 0;
      renderers.push(this);
    }
    setPixelRatio() {}
    setSize() {}
    render(scene) { this.scene = scene; this.renders++; }
    dispose() { this.disposed = true; }
  }
  const context = vm.createContext({
    window, document, performance: { now: () => 0 },
    loadThree: () => Promise.resolve({ ...THREE, WebGLRenderer: Renderer }),
    requestAnimationFrame: callback => { frames.set(++id, callback); return id; },
    cancelAnimationFrame: handle => frames.delete(handle),
  });
  vm.runInContext(source.replaceAll('export function ', 'function ')
    .replace('import("three")', 'loadThree()') +
    '\nglobalThis.api = { mountPano, disposePano };', context);
  return { ...context.api, renderers, frames, window, document, media,
    setFail(value) { fail = value; } };
}

test('disposing before import completion cannot resurrect a panorama', async () => {
  const s = setup();
  const canvas = { isConnected: true };
  s.mountPano(canvas);
  s.disposePano(canvas);
  await flush();
  assert.equal(s.renderers.length, 0);
  assert.equal(s.frames.size, 0);
});

test('a failed renderer can be retried on the same canvas', async () => {
  const s = setup();
  const canvas = { isConnected: true };
  s.setFail(true);
  s.mountPano(canvas);
  await flush();
  s.setFail(false);
  s.mountPano(canvas);
  await flush();
  assert.equal(s.renderers.length, 1);
  s.disposePano(canvas);
});

test('hidden tabs pause and reduced motion repaints on resize without a loop', async () => {
  const s = setup({ reduced: true, hidden: true });
  const canvas = { isConnected: true };
  s.mountPano(canvas);
  await flush();
  const renderer = s.renderers[0];
  assert.equal(renderer.renders, 0);
  assert.equal(s.frames.size, 0);
  s.document.hidden = false;
  s.document.fire('visibilitychange');
  assert.equal(renderer.renders, 1);
  s.window.fire('resize');
  assert.equal(renderer.renders, 2);
  assert.equal(s.frames.size, 0);
  const mesh = renderer.scene.children.find(child => child.isInstancedMesh);
  const disposed = [];
  for (const [name, resource] of [['mesh', mesh], ['geometry', mesh.geometry], ['material', mesh.material]]) {
    resource.addEventListener('dispose', () => disposed.push(name));
  }
  s.disposePano(canvas);
  assert.deepEqual(disposed, ['mesh', 'geometry', 'material']);
  assert.equal(renderer.disposed, true);
  for (const target of [s.window, s.document, s.media]) assert.equal(target.listeners.size, 0);
});
