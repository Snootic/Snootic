let active = null;

export function mountPano(canvas) {
  if (!canvas || active?.canvas === canvas) return;
  if (active) disposePano(active.canvas);
  const mount = { canvas, dispose: () => {} };
  active = mount;
  import("three")
    .then((THREE) => {
      if (active !== mount) return;
      if (!canvas.isConnected) {
        active = null;
        return;
      }
      const dispose = run(THREE, canvas);
      if (dispose) mount.dispose = dispose;
      else active = null;
    })
    .catch(() => {
      if (active === mount) active = null;
    });
}

export function disposePano(canvas) {
  if (active?.canvas !== canvas) return;
  active.dispose();
  active = null;
}

function run(THREE, canvas) {
  let renderer;
  try {
    renderer = new THREE.WebGLRenderer({
      canvas,
      antialias: true,
      alpha: true,
      powerPreference: "low-power",
    });
  } catch {
    return;
  }

  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)");
  const scene = new THREE.Scene();
  scene.fog = new THREE.Fog(0x0a0907, 20, 52);

  const camera = new THREE.PerspectiveCamera(32, 1, 0.1, 160);
  camera.position.set(0, 7.4, 24);
  camera.lookAt(0, 1.1, 0);

  scene.add(new THREE.AmbientLight(0xf0ebe5, 0.5));
  const key = new THREE.DirectionalLight(0xf0ebe5, 0.85);
  key.position.set(6, 13, 7);
  scene.add(key);
  const rim = new THREE.DirectionalLight(0x72a47c, 0.35);
  rim.position.set(-8, 5, -7);
  scene.add(rim);

  const N = 26;
  const hash = (a, b) => {
    const n = Math.sin(a * 12.9898 + b * 78.233) * 43758.5453;
    return n - Math.floor(n);
  };

  const geo = new THREE.BoxGeometry(0.94, 1, 0.94);
  geo.translate(0, 0.5, 0);
  const mesh = new THREE.InstancedMesh(geo, new THREE.MeshLambertMaterial(), N * N);

  const m = new THREE.Matrix4();
  const q = new THREE.Quaternion();
  const pos = new THREE.Vector3();
  const scl = new THREE.Vector3();
  const dark0 = new THREE.Color(0x1e1a17);
  const dark1 = new THREE.Color(0x453c31);
  const orange = new THREE.Color(0xcc7a5a);
  const green = new THREE.Color(0x72a47c);
  const col = new THREE.Color();

  let i = 0;
  for (let x = 0; x < N; x++) {
    for (let z = 0; z < N; z++) {
      const a = hash(x, z);
      const b = hash(z * 1.7 + 5, x * 0.9 + 3);
      const t = (a + b) * 0.5;
      const h = 0.4 + Math.pow(t, 2.7) * 4.6;
      pos.set((x - (N - 1) / 2), 0, (z - (N - 1) / 2));
      scl.set(1, h, 1);
      m.compose(pos, q, scl);
      mesh.setMatrixAt(i, m);

      if (t > 0.955) col.copy(orange).lerp(dark0, 0.14);
      else if (t > 0.905) col.copy(green).lerp(dark0, 0.2);
      else col.copy(dark0).lerp(dark1, Math.min(1, h / 4));
      mesh.setColorAt(i, col);
      i++;
    }
  }
  mesh.instanceMatrix.needsUpdate = true;
  if (mesh.instanceColor) mesh.instanceColor.needsUpdate = true;
  mesh.position.y = -2.8;
  scene.add(mesh);

  const resize = () => {
    const w = window.innerWidth || 1;
    const h = window.innerHeight || 1;
    renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 1.5));
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    if (reduced.matches && !document.hidden) renderer.render(scene, camera);
  };
  resize();
  window.addEventListener("resize", resize, { passive: true });

  let px = 0;
  let py = 0;
  let tx = 0;
  let ty = 0;
  let angle = 0.22;
  let lastInteraction = -Infinity;
  let previousFrame = null;
  let raf = 0;

  const frame = (now) => {
    const previous = previousFrame ?? now;
    previousFrame = now;

    const elapsed = Math.max(0, now - Math.max(previous, lastInteraction + 300)) / 1000;
    if (!reduced.matches) angle += elapsed * (Math.PI * 2 / 120);
    const blend = reduced.matches ? 1 : 1 - Math.exp(-(now - previous) / 470);
    px += (tx - px) * blend;
    py += (ty - py) * blend;
    mesh.rotation.y = angle + px * 0.06;
    camera.position.y = 7.4 - py * 0.7;
    camera.lookAt(0, 1.1, 0);
    renderer.render(scene, camera);
  };
  const loop = (now) => {
    frame(now);
    raf = requestAnimationFrame(loop);
  };

  const play = () => {
    if (raf) return;
    previousFrame = null;
    if (reduced.matches) {
      frame(performance.now());
      return;
    }
    raf = requestAnimationFrame(loop);
  };
  const pause = () => {
    if (raf) {
      cancelAnimationFrame(raf);
      raf = 0;
    }
  };

  const pointermove = (event) => {
    tx = (event.clientX / (window.innerWidth || 1)) * 2 - 1;
    ty = (event.clientY / (window.innerHeight || 1)) * 2 - 1;
    lastInteraction = performance.now();
    if (reduced.matches && !document.hidden) frame(lastInteraction);
  };
  window.addEventListener("pointermove", pointermove, { passive: true });

  if (!document.hidden) play();
  const visibility = () => (document.hidden ? pause() : play());
  const motion = () => {
    pause();
    if (!document.hidden) play();
  };
  document.addEventListener("visibilitychange", visibility);
  reduced.addEventListener("change", motion);
  return () => {
    pause();
    window.removeEventListener("pointermove", pointermove);
    window.removeEventListener("resize", resize);
    document.removeEventListener("visibilitychange", visibility);
    reduced.removeEventListener("change", motion);
    mesh.dispose();
    geo.dispose();
    mesh.material.dispose();
    renderer.dispose();
  };
}
