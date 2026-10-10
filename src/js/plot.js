const BLOCKS = {
  "p-id": [
    ".p-head",
    ".macro",
    ".id-role",
    ".id-tagline",
    ".id-avail",
    ".id-stats",
    ".cta-row",
    ".portrait",
    ".bio",
  ],
  "p-work": [".p-head", ".work-group", ".work-row"],
  "p-stack": [".p-head", ".legend", ".blk"],
  "p-track": [".p-head", ".track-item", ".blk"],
  "p-contact": [".p-head", ".blk"],
};

const WEIGHT = { frame: 0.28, rule: 0.4, hatch: 0.3, type: 1 };

const DURATION = 900;

const BACKSTOP = 1400;

const GLITCH = [
  [0.00, 1.00],
  [0.08, 0.55],
  [0.20, 0.75],
  [0.34, 0.30],
  [0.52, 0.45],
  [0.72, 0.15],
  [1.00, 0.00],
];

function glitchAt(u) {
  for (const [at, v] of GLITCH) if (u <= at) return v;
  return 0;
}

const NS = "http://www.w3.org/2000/svg";
const running = new WeakMap();

const reduced = () =>
  window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches;
const mode = () => document.documentElement.getAttribute("data-motion") || "plot";

const clamp01 = (v) => (v < 0 ? 0 : v > 1 ? 1 : v);
const r2 = (v) => Math.round(v * 100) / 100;

function buildPlan(panel) {
  const svg = panel.querySelector(":scope > .dwg");
  if (!svg) return null;
  svg.replaceChildren();

  const pr = panel.getBoundingClientRect();
  if (pr.width < 40 || pr.height < 40) return null;
  svg.setAttribute("viewBox", `0 0 ${r2(pr.width)} ${r2(pr.height)}`);

  const pad = 5;
  const bw = pr.width;
  const bh = pr.height;
  const segs = [];

  const seg = (x1, y1, x2, y2, el, wt) => {
    const o = { x1, y1, x2, y2, el, wt, s0: 0, w: 0 };
    segs.push(o);
    return o;
  };
  const line = (x1, y1, x2, y2, cls, wt) => {
    const el = document.createElementNS(NS, "path");
    el.setAttribute("d", `M${r2(x1)} ${r2(y1)}L${r2(x2)} ${r2(y2)}`);
    el.setAttribute("pathLength", "1");
    el.setAttribute("class", "st" + (cls ? " " + cls : ""));
    svg.appendChild(el);
    return seg(x1, y1, x2, y2, el, wt);
  };
  const hatch = (x, y, w, h, cls, wt) => {
    if (w <= 0 || h <= 0) return;
    const el = document.createElementNS(NS, "rect");
    el.setAttribute("x", r2(x));
    el.setAttribute("y", r2(y));
    el.setAttribute("width", r2(w));
    el.setAttribute("height", r2(h));
    el.setAttribute("class", "hz" + (cls ? " " + cls : ""));
    svg.appendChild(el);
    return seg(x, y, x + w, y, el, wt);
  };

  line(pad, pad, bw - pad, pad, "st-frame", WEIGHT.frame);
  line(bw - pad, pad, bw - pad, bh - pad, "st-frame", WEIGHT.frame);
  line(bw - pad, bh - pad, pad, bh - pad, "st-frame", WEIGHT.frame);
  line(pad, bh - pad, pad, pad, "st-frame", WEIGHT.frame);

  const inset = 22;
  const ix = pad + inset;
  const iy = pad + inset;
  const iw = bw - 2 * ix;
  const ih = bh - 2 * iy;
  line(ix, iy, ix + iw, iy, "st-thin", WEIGHT.rule);
  line(ix + iw, iy, ix + iw, iy + ih, "st-thin", WEIGHT.rule);
  line(ix + iw, iy + ih, ix, iy + ih, "st-thin", WEIGHT.rule);
  line(ix, iy + ih, ix, iy, "st-thin", WEIGHT.rule);
  const ticks = 6;
  for (let i = 1; i < ticks; i++) {
    const t = ix + (iw * i) / ticks;
    line(t, pad, t, iy, "st-thin", WEIGHT.rule);
    line(t, iy + ih, t, bh - pad, "st-thin", WEIGHT.rule);
  }
  for (let i = 1; i < ticks; i++) {
    const t = iy + (ih * i) / ticks;
    line(pad, t, ix, t, "st-thin", WEIGHT.rule);
    line(ix + iw, t, bw - pad, t, "st-thin", WEIGHT.rule);
  }

  hatch(pad, pad, ix - pad, 6, "", WEIGHT.hatch);
  hatch(ix + iw, pad, bw - pad - (ix + iw), 6, "", WEIGHT.hatch);

  const sels = BLOCKS[panel.id] || [];
  const blocks = [];
  for (const sel of sels) {
    for (const el of panel.querySelectorAll(sel)) {
      if (!blocks.includes(el)) blocks.push(el);
    }
  }
  const boxes = blocks
    .map((el) => ({ el, r: el.getBoundingClientRect() }))
    .filter((b) => b.r.width > 2 && b.r.height > 2)
    .map((b) => ({
      el: b.el,
      x: b.r.left - pr.left,
      y: b.r.top - pr.top,
      w: b.r.width,
      h: b.r.height,
    }))
    .sort((a, b) => a.y - b.y || a.x - b.x);

  const rows = [];
  for (const b of boxes) {
    const row = rows[rows.length - 1];
    if (row && b.y < row.bottom - Math.min(12, row.height * 0.5)) {
      row.items.push(b);
      row.bottom = Math.max(row.bottom, b.y + b.h);
      row.height = row.bottom - row.top;
    } else {
      rows.push({
        items: [b],
        top: b.y,
        bottom: b.y + b.h,
        height: b.h,
      });
    }
  }
  for (const row of rows) row.items.sort((a, b) => a.x - b.x);

  const annotated = [];
  for (const row of rows) {
    const left = Math.min(...row.items.map((b) => b.x));
    const right = Math.max(...row.items.map((b) => b.x + b.w));
    const above = Math.max(iy, row.top - 7);
    line(left, above, right, above, "st-thin", WEIGHT.rule);
    for (const b of row.items) {
      seg(b.x, b.y + b.h / 2, b.x + b.w, b.y + b.h / 2, b.el, WEIGHT.type);
      annotated.push(b.el);
    }
  }

  hatch(ix, iy + ih + 6, iw, Math.max(0, bh - pad - (iy + ih + 6)), "", WEIGHT.hatch);

  let total = 0;
  for (const s of segs) total += s.wt;
  let acc = 0;
  for (const s of segs) {
    s.s0 = (acc / total) * DURATION;
    s.w = (s.wt / total) * DURATION;
    acc += s.wt;
  }

  return { segs, content: annotated };
}

function unset(el, prop) {
  if (!el || !el.style) return;
  el.style.removeProperty(prop);
  if (el.getAttribute("style") === "") el.removeAttribute("style");
}

function settle(panel) {
  const svg = panel.querySelector(":scope > .dwg");
  if (svg) svg.replaceChildren();
  panel.removeAttribute("data-plot");
  unset(panel, "--gx");
  for (const el of panel.querySelectorAll("[data-blk]")) {
    el.removeAttribute("data-blk");
    unset(el, "--q");
  }
  const pen = panel.querySelector(":scope > .pen");
  if (pen) {
    pen.classList.remove("is-live");
    unset(pen, "--px");
    unset(pen, "--py");
    unset(pen, "--tw");
  }
}

export function plotSheet(panel) {
  if (!panel) return;
  const prior = running.get(panel);
  if (prior) prior.abort();

  if (reduced() || mode() !== "plot") {
    settle(panel);
    return;
  }

  const plan = buildPlan(panel);
  if (!plan) {
    settle(panel);
    return;
  }

  const { segs, content } = plan;
  panel.setAttribute("data-plot", "");

  for (const s of segs) set_q(s, 0);
  for (const el of content) el.setAttribute("data-blk", "");

  const pen = panel.querySelector(":scope > .pen");
  if (pen) pen.classList.add("is-live");

  let raf = 0;
  let backstop = 0;
  let done = false;
  let cursor = -1;
  let gx = -1;

  const paint = (t) => {

    const g = glitchAt(t / DURATION);
    if (g !== gx) {
      panel.style.setProperty("--gx", String(r2(g)));
      gx = g;
    }

    let i = cursor;
    while (i + 1 < segs.length && t >= segs[i + 1].s0) i++;
    for (let k = Math.max(0, cursor); k < i; k++) {
      set_q(segs[k], 1);
    }
    if (i >= 0) {
      const s = segs[i];
      const q = clamp01((t - s.s0) / s.w);
      set_q(s, q);
      if (pen) {
        const x = s.x1 + (s.x2 - s.x1) * q;
        const y = s.y1 + (s.y2 - s.y1) * q;
        pen.style.setProperty("--px", `${r2(x)}px`);
        pen.style.setProperty("--py", `${r2(y)}px`);
        const dx = Math.abs(s.x2 - s.x1);
        pen.style.setProperty("--tw", `${r2(q * (dx > 1 ? dx : 0))}px`);
      }
      cursor = i;
    }
  };

  const finish = () => {
    if (done) return;
    done = true;
    running.delete(panel);
    if (raf) cancelAnimationFrame(raf);
    if (backstop) clearTimeout(backstop);
    settle(panel);
  };

  const start = performance.now();
  const step = (now) => {
    if (done) return;
    const t = now - start;
    if (t >= DURATION) {
      finish();
      return;
    }
    paint(t);
    raf = requestAnimationFrame(step);
  };

  running.set(panel, { abort: finish });

  raf = requestAnimationFrame(step);
  backstop = setTimeout(() => {
    paint(DURATION);
    finish();
  }, DURATION + BACKSTOP);
}

function set_q(s, q) {
  if (!s.el || s.el === document.body) return;
  s.el.style.setProperty("--q", String(clamp01(q)));
}

export function clearPlot(panel) {
  if (!panel) return;
  const prior = running.get(panel);
  if (prior) {
    prior.abort();
    return;
  }
  settle(panel);
}
