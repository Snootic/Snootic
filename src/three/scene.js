import * as THREE from "three";
import { WebGPURenderer } from "three/webgpu";
import { createCard, createCardTexture, updateCardDot } from "./sceneCard.js";

class Scene {
  bgRenderer;
  fgRenderer;
  scene;
  fgCanvas;
  bgCanvas;
  camera;
  clock;

  targetX = 0;
  targetY = 0;

  STARSCOUNT = 300;
  BRAND = new THREE.Color("#CC7A5A");
  MID = new THREE.Color("#6E6560");
  DIM = new THREE.Color("#2E2924");

  constructor(fgCanvas, bgCanvas) {
    this.scene = new THREE.Scene();
    this.fgCanvas = fgCanvas;
    this.bgCanvas = bgCanvas;

    this.clock = new THREE.Timer();

    window.addEventListener("mousemove", (e) => {
      this.targetX = (e.clientX / window.innerWidth) * 2 - 1;
      this.targetY = -(e.clientY / window.innerHeight) * 2 + 1;
    });
    window.addEventListener("resize", () => {
      const width = this.fgCanvas.clientWidth;
      const height = this.fgCanvas.clientHeight;

      this.camera.aspect = width / height;
      this.camera.updateProjectionMatrix();

      this.bgRenderer.setSize(width, height, false);
      this.fgRenderer.setSize(width, height, false);
    });
  }

  async init() {
    if (navigator.gpu) {
      this.bgRenderer = new WebGPURenderer({
        canvas: this.bgCanvas,
        alpha: true,
      });
      this.fgRenderer = new WebGPURenderer({
        canvas: this.fgCanvas,
        alpha: true,
      });
      await Promise.all([this.bgRenderer.init(), this.fgRenderer.init()]);
    } else {
      this.bgRenderer = new THREE.WebGLRenderer({
        canvas: this.bgCanvas,
        alpha: true,
      });
      this.fgRenderer = new THREE.WebGLRenderer({
        canvas: this.fgCanvas,
        alpha: true,
      });
    }

    this.bgRenderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    this.fgRenderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));

    this.bgRenderer.setSize(
      this.fgCanvas.clientWidth,
      this.fgCanvas.clientHeight,
      false,
    );
    this.fgRenderer.setSize(
      this.fgCanvas.clientWidth,
      this.fgCanvas.clientHeight,
      false,
    );

    this.bgRenderer.setClearColor(new THREE.Color("#000000"), 0);
    this.fgRenderer.setClearColor(new THREE.Color("#000000"), 0);
  }

  setupCamera() {
    this.camera = new THREE.PerspectiveCamera(
      60,
      this.fgCanvas.clientWidth / this.fgCanvas.clientHeight,
      0.1,
      100,
    );
    this.camera.position.z = 6;

    const ambient = new THREE.AmbientLight(0xfff5ee, 0.7);
    const keyLight = new THREE.DirectionalLight(0xffe0cc, 1.4);
    keyLight.position.set(4, 6, 5);
    const rimLight = new THREE.DirectionalLight(0xcc7a5a, 0.5);
    rimLight.position.set(-5, -2, -4);

    this.scene.add(ambient, keyLight, rimLight);
  }

  mountStars(group, count) {
    const positions = new Float32Array(count * 3);
    const colors = new Float32Array(count * 3);

    for (let i = 0; i < count; i++) {
      const theta = Math.random() * Math.PI * 2;
      const phi = Math.acos(2 * Math.random() - 1);
      const r = 2.5 + Math.random() * 2.5;

      positions[i * 3] = r * Math.sin(phi) * Math.cos(theta);
      positions[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta);
      positions[i * 3 + 2] = r * Math.cos(phi);

      const rand = Math.random();
      const color =
        rand < 0.25 ? this.BRAND : rand < 0.65 ? this.MID : this.DIM;
      colors[i * 3] = color.r;
      colors[i * 3 + 1] = color.g;
      colors[i * 3 + 2] = color.b;
    }

    const geo = new THREE.BufferGeometry();
    geo.setAttribute("position", new THREE.BufferAttribute(positions, 3));
    geo.setAttribute("color", new THREE.BufferAttribute(colors, 3));

    const mat = new THREE.PointsMaterial({
      size: 0.045,
      vertexColors: true,
      transparent: true,
      opacity: 0.8,
      blending: THREE.AdditiveBlending,
      depthWrite: false,
      sizeAttenuation: true,
    });

    const stars = new THREE.Points(geo, mat);
    stars.name = "stars";
    group.add(stars);
  }
}

export async function HeroScene(fgCanvas, bgCanvas, personal) {
  const heroScene = new Scene(fgCanvas, bgCanvas);
  await heroScene.init();
  heroScene.setupCamera();
  heroScene.bgGroup = new THREE.Group();
  heroScene.fgGroup = new THREE.Group();
  heroScene.scene.add(heroScene.bgGroup, heroScene.fgGroup);

  heroScene.mountStars(heroScene.bgGroup, 90);
  heroScene.mountStars(heroScene.fgGroup, 90);
  
  const {cardCanvas, ctx} = createCard(personal.name, personal.title, personal.availability, 12, 36);
  const cardTexture = createCardTexture(cardCanvas);
  
  const cardGeo = new THREE.PlaneGeometry(3, 3 * (200 / 340));
  const cardMat = new THREE.MeshBasicMaterial({
    map: cardTexture,
    transparent: true,
    opacity: 0.95,
  });
  const cardMesh = new THREE.Mesh(cardGeo, cardMat);
  cardMesh.position.set(0, 1.5, 0.6);
  heroScene.scene.add(cardMesh);
  
  const raycaster = new THREE.Raycaster();
  const mouse = new THREE.Vector2();
  
  window.addEventListener('click', (e) => {
    mouse.x = (e.clientX / window.innerWidth) * 2 - 1;
    mouse.y = -(e.clientY / window.innerHeight) * 2 + 1;
  
    raycaster.setFromCamera(mouse, heroScene.camera);
    const intersects = raycaster.intersectObject(cardMesh);
  
    if (intersects.length > 0) {
      console.log("Card Clicked!");
      //TODO: make card "language" changes when pressing the colored buttons
      // red: rust
      // yellow: python
      // blue: ts
    }
  });


  function animate() {
    heroScene.clock.update();
    const elapsed = heroScene.clock.getElapsed();

    for (const group of [heroScene.bgGroup, heroScene.fgGroup]) {
      const stars = group.getObjectByName("stars");
      const glow = 0.8 + Math.sin(elapsed * 1.5) * 0.1;
      const newSize = 0.045 + Math.sin(elapsed * 1.5) * 0.005;
      if (stars) {
        stars.material.size = newSize;
        stars.material.opacity = glow;
        stars.rotation.y = elapsed * 0.04;
        stars.rotation.x = Math.sin(elapsed * 0.07) * 0.1;
      }
    }

    heroScene.camera.position.x +=
      (heroScene.targetX * 0.8 - heroScene.camera.position.x) * 0.04;
    heroScene.camera.position.y +=
      (heroScene.targetY * 0.8 - heroScene.camera.position.y) * 0.04;
    heroScene.camera.lookAt(heroScene.scene.position);

    updateCardDot(ctx, 12, elapsed)

    cardMesh.material.map.needsUpdate = true;
    cardTexture.needsUpdate = true;

    const isMobile = window.innerWidth < 768;
    cardMesh.scale.setScalar(isMobile ? 0.9 : 1.0);
    cardMesh.position.y =
      (isMobile ? -1.4 : -1.3) + Math.sin(elapsed * 1.5) * 0.05;

    heroScene.fgGroup.visible = false;
    heroScene.bgGroup.visible = true;
    cardMesh.visible = true;
    heroScene.bgRenderer.render(heroScene.scene, heroScene.camera);

    heroScene.bgGroup.visible = false;
    cardMesh.visible = false;
    heroScene.fgGroup.visible = true;
    heroScene.fgRenderer.render(heroScene.scene, heroScene.camera);
  }

  heroScene.bgRenderer.setAnimationLoop(animate);
}
