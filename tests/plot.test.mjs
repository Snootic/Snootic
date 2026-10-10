import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import vm from 'node:vm';

const source = await readFile(new URL('../src/js/plot.js', import.meta.url), 'utf8');

function setup({ reduced = false, mode = 'plot' } = {}) {
  const frames = new Map();
  const timers = new Map();
  let id = 0;
  const element = () => ({
    attributes: new Map(),
    children: [],
    style: {
      values: new Map(),
      setProperty(key, value) { this.values.set(key, value); },
      removeProperty(key) { this.values.delete(key); },
    },
    classList: {
      values: new Set(),
      add(value) { this.values.add(value); },
      remove(value) { this.values.delete(value); },
    },
    setAttribute(key, value) { this.attributes.set(key, value); },
    getAttribute(key) { return this.attributes.get(key); },
    removeAttribute(key) { this.attributes.delete(key); },
    replaceChildren() { this.children = []; },
    appendChild(child) { this.children.push(child); },
  });
  const svg = element();
  const pen = element();
  const block = element();
  block.getBoundingClientRect = () => ({ left: 120, top: 240, width: 160, height: 30 });
  const panel = element();
  panel.id = 'p-contact';
  panel.getBoundingClientRect = () => ({ left: 100, top: 200, width: 400, height: 300 });
  panel.querySelector = selector => selector.endsWith('.dwg') ? svg : pen;
  panel.querySelectorAll = selector => selector === '.blk' ||
    (selector === '[data-blk]' && block.attributes.has('data-blk')) ? [block] : [];
  const context = vm.createContext({
    window: { matchMedia: () => ({ matches: reduced }) },
    document: {
      documentElement: { getAttribute: () => mode },
      createElementNS: element,
    },
    performance: { now: () => 0 },
    requestAnimationFrame: callback => { frames.set(++id, callback); return id; },
    cancelAnimationFrame: handle => frames.delete(handle),
    setTimeout: callback => { timers.set(++id, callback); return id; },
    clearTimeout: handle => timers.delete(handle),
  });
  vm.runInContext(source.replaceAll('export function ', 'function ') +
    '\nglobalThis.api = { plotSheet, clearPlot };', context);
  const frame = time => {
    const [handle, callback] = frames.entries().next().value;
    frames.delete(handle);
    callback(time);
  };
  return { ...context.api, panel, pen, block, svg, frames, timers, frame };
}

function assertSettled(s) {
  assert.equal(s.frames.size, 0);
  assert.equal(s.timers.size, 0);
  assert.equal(s.svg.children.length, 0);
  assert.equal(s.panel.attributes.has('data-plot'), false);
  assert.equal(s.block.attributes.has('data-blk'), false);
  assert.equal(s.block.style.values.size, 0);
  assert.equal(s.pen.style.values.size, 0);
  assert.equal(s.pen.classList.values.has('is-live'), false);
}

test('pen coordinates remain relative to an offset panel', () => {
  const s = setup();
  s.plotSheet(s.panel);
  s.frame(0);
  assert.equal(s.pen.style.values.get('--px'), '5px');
  assert.equal(s.pen.style.values.get('--py'), '5px');
  s.clearPlot(s.panel);
  assertSettled(s);
});

test('restarting, completing and clearing a plot release all scheduled work', () => {
  const s = setup();
  s.plotSheet(s.panel);
  s.plotSheet(s.panel);
  assert.equal(s.frames.size, 1);
  assert.equal(s.timers.size, 1);
  s.frame(900);
  assertSettled(s);
  s.clearPlot(s.panel);
  assertSettled(s);
});

test('the timeout reveals content when animation frames never run', () => {
  const s = setup();
  s.plotSheet(s.panel);
  s.timers.values().next().value();
  assertSettled(s);
});

test('reduced motion and mechanical mode leave content visible', () => {
  for (const options of [{ reduced: true }, { mode: 'mech' }]) {
    const s = setup(options);
    s.plotSheet(s.panel);
    assertSettled(s);
  }
});

test('completed strokes reach full opacity when the pen moves on', () => {
  const s = setup();
  s.plotSheet(s.panel);
  s.frame(0);
  assert.equal(s.svg.children[0].style.values.get('--q'), '0');
  s.frame(300);
  assert.equal(s.svg.children[0].style.values.get('--q'), '1');
  s.clearPlot(s.panel);
});
