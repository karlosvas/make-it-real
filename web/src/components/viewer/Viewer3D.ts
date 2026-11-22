// Visor 3D: reproduce la traza que devuelve mir-server.
// Unreal (X delante, Y derecha, Z arriba, cm) → three.js (x, y arriba, z): (X, Z, Y) y rotation.y = -Yaw.
import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { ACTOR_STRIDE, type ActorMeta, type DebugDraw, type Trace } from '../../lib/types';

const DEG = Math.PI / 180;
export const ue = (x: number, y: number, z: number) => new THREE.Vector3(x, z, y);

interface ActorObj {
  meta: ActorMeta;
  group: THREE.Group;
  leafPivot?: THREE.Group;
  cone?: THREE.Mesh;
  coneMat?: THREE.MeshBasicMaterial;
  spin?: THREE.Object3D;
  bodyMats?: THREE.MeshStandardMaterial[];
}

interface Label {
  pos: THREE.Vector3;
  text: string;
  color: string;
  kind: 'bubble' | 'tag' | 'debug';
}

function textSprite(text: string, color: string, scale = 1) {
  const canvas = document.createElement('canvas');
  const ctx = canvas.getContext('2d')!;
  const font = '600 44px "JetBrains Mono", monospace';
  ctx.font = font;
  const w = Math.ceil(ctx.measureText(text).width) + 40;
  canvas.width = w;
  canvas.height = 72;
  ctx.font = font;
  ctx.fillStyle = 'rgba(8,10,14,0.72)';
  ctx.beginPath();
  ctx.roundRect(0, 0, w, 72, 18);
  ctx.fill();
  ctx.fillStyle = color;
  ctx.textBaseline = 'middle';
  ctx.fillText(text, 20, 38);
  const tex = new THREE.CanvasTexture(canvas);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.anisotropy = 4;
  const sprite = new THREE.Sprite(new THREE.SpriteMaterial({ map: tex, depthWrite: false, transparent: true }));
  sprite.scale.set(w * 0.9 * scale, 72 * 0.9 * scale, 1);
  return sprite;
}

function std(color: number, extra: Partial<THREE.MeshStandardMaterialParameters> = {}) {
  return new THREE.MeshStandardMaterial({ color, roughness: 0.55, metalness: 0.05, ...extra });
}

function shadowed<T extends THREE.Object3D>(o: T): T {
  o.traverse((c) => {
    if ((c as THREE.Mesh).isMesh) {
      c.castShadow = true;
      c.receiveShadow = true;
    }
  });
  return o;
}

function makeCharacter(body: number, accent: number): { group: THREE.Group; mats: THREE.MeshStandardMaterial[] } {
  const g = new THREE.Group();
  const bodyMat = std(body);
  const accentMat = std(accent, { emissive: accent, emissiveIntensity: 0.35 });
  const torso = new THREE.Mesh(new THREE.CapsuleGeometry(27, 78, 6, 16), bodyMat);
  torso.position.y = 27 + 39 + 8;
  const head = new THREE.Mesh(new THREE.SphereGeometry(21, 24, 16), bodyMat);
  head.position.y = 142;
  // "Visor": deja claro hacia dónde mira (su forward, +X en Unreal).
  const visor = new THREE.Mesh(new THREE.BoxGeometry(14, 9, 28), accentMat);
  visor.position.set(17, 145, 0);
  const nose = new THREE.Mesh(new THREE.ConeGeometry(9, 26, 12), accentMat);
  nose.rotation.z = -Math.PI / 2;
  nose.position.set(42, 4, 0);
  const ring = new THREE.Mesh(
    new THREE.RingGeometry(30, 36, 32),
    new THREE.MeshBasicMaterial({ color: accent, transparent: true, opacity: 0.55, side: THREE.DoubleSide }),
  );
  ring.rotation.x = -Math.PI / 2;
  ring.position.y = 1.5;
  g.add(torso, head, visor, nose, ring);
  shadowed(torso);
  shadowed(head);
  return { group: g, mats: [bodyMat] };
}

function makeDoor(meta: ActorMeta): { group: THREE.Group; leafPivot: THREE.Group } {
  const g = new THREE.Group();
  const frameMat = std(0x9aa3b5, { roughness: 0.7 });
  const leafMat = std(0xb45309, { roughness: 0.6 });
  for (const side of [-1, 1]) {
    const post = shadowed(new THREE.Mesh(new THREE.BoxGeometry(16, 232, 14), frameMat));
    post.position.set(0, 116, side * 67);
    g.add(post);
  }
  const lintel = shadowed(new THREE.Mesh(new THREE.BoxGeometry(16, 16, 150), frameMat));
  lintel.position.set(0, 232, 0);
  g.add(lintel);

  const leafPivot = new THREE.Group();
  leafPivot.position.set(0, 0, -60);
  const leaf = shadowed(new THREE.Mesh(new THREE.BoxGeometry(7, 214, 118), leafMat));
  leaf.position.set(0, 108, 59);
  const knob = new THREE.Mesh(new THREE.SphereGeometry(5, 12, 8), std(0xfacc15, { metalness: 0.8, roughness: 0.3 }));
  knob.position.set(8, 104, 104);
  const knob2 = knob.clone();
  knob2.position.x = -8;
  leafPivot.add(leaf, knob, knob2);
  g.add(leafPivot);

  if (meta.walls) {
    const wallMat = std(0x5b6478, { roughness: 0.85 });
    for (const side of [-1, 1]) {
      const wall = shadowed(new THREE.Mesh(new THREE.BoxGeometry(20, 250, 350), wallMat));
      wall.position.set(0, 125, side * 235);
      g.add(wall);
    }
  }

  // Lado de delante (verde) y de detrás (rojo).
  const arrow = new THREE.Shape();
  arrow.moveTo(0, -40);
  arrow.lineTo(70, -40);
  arrow.lineTo(70, -70);
  arrow.lineTo(130, 0);
  arrow.lineTo(70, 70);
  arrow.lineTo(70, 40);
  arrow.lineTo(0, 40);
  arrow.closePath();
  const front = new THREE.Mesh(
    new THREE.ShapeGeometry(arrow),
    new THREE.MeshBasicMaterial({ color: 0x22c55e, transparent: true, opacity: 0.5, depthWrite: false }),
  );
  front.rotation.x = -Math.PI / 2;
  front.position.set(30, 1, 0);
  const frontLabel = textSprite('DELANTE', '#4ade80', 0.85);
  frontLabel.position.set(210, 26, 0);
  const backPlate = new THREE.Mesh(
    new THREE.PlaneGeometry(90, 120),
    new THREE.MeshBasicMaterial({ color: 0xef4444, transparent: true, opacity: 0.18, depthWrite: false }),
  );
  backPlate.rotation.x = -Math.PI / 2;
  backPlate.position.set(-70, 1, 0);
  const backLabel = textSprite('DETRÁS', '#f87171', 0.85);
  backLabel.position.set(-170, 26, 0);
  g.add(front, frontLabel, backPlate, backLabel);
  return { group: g, leafPivot };
}

function makeKey(): { group: THREE.Group; spin: THREE.Object3D } {
  const g = new THREE.Group();
  const spin = new THREE.Group();
  const gold = std(0xfacc15, { metalness: 0.85, roughness: 0.25, emissive: 0x7c5a00, emissiveIntensity: 0.5 });
  const ring = new THREE.Mesh(new THREE.TorusGeometry(13, 4.5, 12, 28), gold);
  ring.position.y = 0;
  const shaft = new THREE.Mesh(new THREE.BoxGeometry(5, 34, 5), gold);
  shaft.position.y = -30;
  const tooth = new THREE.Mesh(new THREE.BoxGeometry(5, 6, 12), gold);
  tooth.position.set(0, -42, 7);
  spin.add(ring, shaft, tooth);
  spin.position.y = 70;
  shadowed(spin);
  const glow = new THREE.Mesh(
    new THREE.CircleGeometry(34, 32),
    new THREE.MeshBasicMaterial({ color: 0xfacc15, transparent: true, opacity: 0.18, depthWrite: false }),
  );
  glow.rotation.x = -Math.PI / 2;
  glow.position.y = 1;
  g.add(spin, glow);
  return { group: g, spin };
}

function makeTargetPoint(label: string): THREE.Group {
  const g = new THREE.Group();
  const mat = new THREE.MeshBasicMaterial({ color: 0x60a5fa, transparent: true, opacity: 0.7 });
  const disc = new THREE.Mesh(new THREE.RingGeometry(16, 24, 32), mat);
  disc.rotation.x = -Math.PI / 2;
  disc.position.y = 1.5;
  const pole = new THREE.Mesh(new THREE.CylinderGeometry(1.5, 1.5, 60, 6), mat);
  pole.position.y = 30;
  const tag = textSprite(label, '#93c5fd', 0.6);
  tag.position.y = 78;
  g.add(disc, pole, tag);
  return g;
}

export class Viewer3D {
  private renderer: THREE.WebGLRenderer;
  private scene = new THREE.Scene();
  private camera: THREE.PerspectiveCamera;
  private controls: OrbitControls;
  private overlay: HTMLDivElement;
  private screenBox: HTMLDivElement;
  private actorsRoot = new THREE.Group();
  private debug: THREE.LineSegments;
  private actors: ActorObj[] = [];
  private trace: Trace | null = null;
  private time = 0;
  private raf = 0;
  private labelPool: HTMLDivElement[] = [];
  private resizeObs: ResizeObserver;
  private lastFitKey = '';

  constructor(private container: HTMLElement) {
    this.renderer = new THREE.WebGLRenderer({ antialias: true });
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    this.renderer.outputColorSpace = THREE.SRGBColorSpace;
    this.renderer.toneMapping = THREE.ACESFilmicToneMapping;
    this.renderer.toneMappingExposure = 1.05;
    this.renderer.shadowMap.enabled = true;
    this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    this.renderer.domElement.style.display = 'block';
    container.appendChild(this.renderer.domElement);

    this.overlay = document.createElement('div');
    this.overlay.style.cssText = 'position:absolute;inset:0;pointer-events:none;overflow:hidden;';
    container.appendChild(this.overlay);
    this.screenBox = document.createElement('div');
    this.screenBox.style.cssText = 'position:absolute;left:12px;top:10px;display:flex;flex-direction:column;gap:2px;font:500 12px "JetBrains Mono",monospace;text-shadow:0 1px 2px #000;pointer-events:none;';
    container.appendChild(this.screenBox);

    this.camera = new THREE.PerspectiveCamera(42, 1, 5, 40000);
    this.camera.position.set(300, 900, 1100);
    this.controls = new OrbitControls(this.camera, this.renderer.domElement);
    this.controls.enableDamping = true;
    this.controls.maxPolarAngle = Math.PI * 0.48;
    this.controls.minDistance = 150;
    this.controls.maxDistance = 6000;

    const bg = new THREE.Color(0x0d1016);
    this.scene.background = bg;
    this.scene.fog = new THREE.Fog(bg, 2500, 7000);
    this.scene.add(new THREE.HemisphereLight(0xdbe4ff, 0x2a2f3a, 1.9));
    const sun = new THREE.DirectionalLight(0xfff4e0, 3.0);
    sun.position.set(-700, 1400, 900);
    sun.castShadow = true;
    sun.shadow.mapSize.set(2048, 2048);
    const sc = sun.shadow.camera;
    sc.left = -1600;
    sc.right = 1600;
    sc.top = 1600;
    sc.bottom = -1600;
    sc.near = 100;
    sc.far = 4000;
    sun.shadow.bias = -0.0005;
    this.scene.add(sun);

    const floor = new THREE.Mesh(new THREE.PlaneGeometry(9000, 9000), std(0x1f2430, { roughness: 1 }));
    floor.rotation.x = -Math.PI / 2;
    floor.receiveShadow = true;
    this.scene.add(floor);
    const grid = new THREE.GridHelper(9000, 90, 0x2f3646, 0x222733);
    grid.position.y = 0.5;
    this.scene.add(grid);
    const axes = new THREE.Group();
    const xAxis = new THREE.Mesh(new THREE.BoxGeometry(160, 1, 3), new THREE.MeshBasicMaterial({ color: 0xef4444 }));
    xAxis.position.set(80, 1, 0);
    const yAxis = new THREE.Mesh(new THREE.BoxGeometry(3, 1, 160), new THREE.MeshBasicMaterial({ color: 0x22c55e }));
    yAxis.position.set(0, 1, 80);
    axes.add(xAxis, yAxis);
    axes.position.set(0, 0, 0);
    this.scene.add(axes);

    this.scene.add(this.actorsRoot);
    const debugGeo = new THREE.BufferGeometry();
    this.debug = new THREE.LineSegments(debugGeo, new THREE.LineBasicMaterial({ vertexColors: true, depthTest: false, transparent: true }));
    this.debug.renderOrder = 10;
    this.debug.frustumCulled = false;
    this.scene.add(this.debug);

    this.resizeObs = new ResizeObserver(() => this.resize());
    this.resizeObs.observe(container);
    this.resize();
    const loop = () => {
      this.raf = requestAnimationFrame(loop);
      this.frame();
    };
    loop();
  }

  dispose() {
    cancelAnimationFrame(this.raf);
    this.resizeObs.disconnect();
    this.controls.dispose();
    this.renderer.dispose();
    this.renderer.domElement.remove();
    this.overlay.remove();
    this.screenBox.remove();
  }

  private resize() {
    const w = Math.max(1, this.container.clientWidth);
    const h = Math.max(1, this.container.clientHeight);
    this.renderer.setSize(w, h, false);
    this.renderer.domElement.style.width = `${w}px`;
    this.renderer.domElement.style.height = `${h}px`;
    this.camera.aspect = w / h;
    this.camera.updateProjectionMatrix();
  }

  setTrace(trace: Trace | null) {
    this.trace = trace;
    this.actorsRoot.clear();
    this.actors = [];
    if (!trace) return;
    for (const meta of trace.actors) {
      let obj: ActorObj;
      if (meta.class === 'monigote') {
        const c = makeCharacter(0x38bdf8, 0xff8f4d);
        obj = { meta, group: c.group, bodyMats: c.mats };
      } else if (meta.class === 'guard') {
        const c = makeCharacter(0xdc2626, 0xf5f5f5);
        obj = { meta, group: c.group, bodyMats: c.mats };
        if (meta.sight) {
          const [radius, half] = meta.sight;
          const mat = new THREE.MeshBasicMaterial({ color: 0x22c55e, transparent: true, opacity: 0.16, side: THREE.DoubleSide, depthWrite: false });
          const cone = new THREE.Mesh(new THREE.CircleGeometry(radius, 48, -half * DEG, 2 * half * DEG), mat);
          cone.rotation.x = -Math.PI / 2;
          cone.position.y = 2;
          obj.group.add(cone);
          obj.cone = cone;
          obj.coneMat = mat;
        }
      } else if (meta.class === 'door') {
        const d = makeDoor(meta);
        obj = { meta, group: d.group, leafPivot: d.leafPivot };
      } else if (meta.class === 'key') {
        const k = makeKey();
        obj = { meta, group: k.group, spin: k.spin };
        if (meta.label) {
          const tag = textSprite(meta.label, '#fde68a', 0.5);
          tag.position.y = 118;
          obj.group.add(tag);
        }
      } else if (meta.class === 'target_point') {
        obj = { meta, group: makeTargetPoint(meta.label ?? meta.name) };
      } else {
        const g = new THREE.Group();
        g.add(shadowed(new THREE.Mesh(new THREE.BoxGeometry(40, 40, 40), std(0x9ca3af))));
        obj = { meta, group: g };
      }
      this.actorsRoot.add(obj.group);
      this.actors.push(obj);
    }
    const key = trace.actors.map((a) => a.class).join(',') + trace.frames[0]?.slice(1, 4).join(',');
    if (key !== this.lastFitKey) {
      this.lastFitKey = key;
      this.fit();
    }
    this.apply();
  }

  /** Encuadra la cámara para ver todo lo que pasa en la traza. */
  fit() {
    const t = this.trace;
    if (!t || t.frames.length === 0) return;
    let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
    for (const f of t.frames) {
      for (let i = 0; i < t.actors.length; i++) {
        const x = f[1 + i * ACTOR_STRIDE];
        const y = f[2 + i * ACTOR_STRIDE];
        minX = Math.min(minX, x); maxX = Math.max(maxX, x);
        minY = Math.min(minY, y); maxY = Math.max(maxY, y);
      }
    }
    const cx = (minX + maxX) / 2;
    const cy = (minY + maxY) / 2;
    const span = Math.max(maxX - minX, maxY - minY, 500);
    const d = span * 1.05 + 500;
    const target = ue(cx, cy, 60);
    this.controls.target.copy(target);
    this.camera.position.copy(target).add(new THREE.Vector3(d * 0.12, d * 0.78, d * 0.78));
    this.controls.update();
  }

  setTime(t: number) {
    this.time = t;
  }

  private sample(): { idx: number; alpha: number } {
    const tr = this.trace!;
    const n = tr.frames.length;
    if (n === 0) return { idx: 0, alpha: 0 };
    const f = Math.max(0, this.time / tr.dt);
    const idx = Math.min(n - 1, Math.floor(f));
    const alpha = idx >= n - 1 ? 0 : Math.min(1, f - idx);
    return { idx, alpha };
  }

  private apply() {
    const tr = this.trace;
    if (!tr || tr.frames.length === 0) return;
    const { idx, alpha } = this.sample();
    const a = tr.frames[idx];
    const b = tr.frames[Math.min(idx + 1, tr.frames.length - 1)];
    const lerpAngle = (x: number, y: number) => {
      let d = ((y - x + 540) % 360) - 180;
      return x + d * alpha;
    };
    this.actors.forEach((obj, i) => {
      const o = 1 + i * ACTOR_STRIDE;
      const x = a[o] + (b[o] - a[o]) * alpha;
      const y = a[o + 1] + (b[o + 1] - a[o + 1]) * alpha;
      const z = a[o + 2] + (b[o + 2] - a[o + 2]) * alpha;
      const yaw = lerpAngle(a[o + 3], b[o + 3]);
      const leaf = lerpAngle(a[o + 4], b[o + 4]);
      const hidden = a[o + 5] > 0.5;
      const flag = a[o + 6];
      obj.group.position.copy(ue(x, y, z));
      obj.group.rotation.y = -yaw * DEG;
      obj.group.visible = !hidden;
      if (obj.leafPivot) obj.leafPivot.rotation.y = -leaf * DEG;
      if (obj.spin) {
        obj.spin.rotation.y = this.time * 2.2;
        obj.spin.position.y = 70 + Math.sin(this.time * 3) * 6;
      }
      if (obj.coneMat) {
        obj.coneMat.color.set(flag > 0.5 ? 0xef4444 : 0x22c55e);
        obj.coneMat.opacity = flag > 0.5 ? 0.3 : 0.16;
      }
    });
    this.updateDebug();
  }

  private activeDraws(): DebugDraw[] {
    const tr = this.trace;
    if (!tr) return [];
    const t = this.time;
    const eps = 1 / 30 + 1e-3;
    const out: DebugDraw[] = [];
    for (const d of tr.draws) {
      if (d.t > t + 1e-6) continue;
      if (d.persistent || (d.life > 0 && t < d.t + d.life) || (d.life <= 0 && t - d.t < eps)) out.push(d);
      if (out.length > 3000) break;
    }
    return out;
  }

  private updateDebug() {
    const pos: number[] = [];
    const col: number[] = [];
    const c = new THREE.Color();
    const seg = (p: THREE.Vector3, q: THREE.Vector3) => {
      pos.push(p.x, p.y, p.z, q.x, q.y, q.z);
      col.push(c.r, c.g, c.b, c.r, c.g, c.b);
    };
    const circle = (center: THREE.Vector3, r: number, u: THREE.Vector3, v: THREE.Vector3, n = 32) => {
      for (let i = 0; i < n; i++) {
        const a0 = (i / n) * Math.PI * 2;
        const a1 = ((i + 1) / n) * Math.PI * 2;
        const p = center.clone().addScaledVector(u, Math.cos(a0) * r).addScaledVector(v, Math.sin(a0) * r);
        const q = center.clone().addScaledVector(u, Math.cos(a1) * r).addScaledVector(v, Math.sin(a1) * r);
        seg(p, q);
      }
    };
    for (const d of this.activeDraws()) {
      if (d.kind === 9) continue;
      c.set(d.color);
      const A = ue(...d.a);
      const B = ue(...d.b);
      switch (d.kind) {
        case 0:
          seg(A, B);
          break;
        case 1: {
          const s = Math.max(4, d.size);
          seg(A.clone().add(new THREE.Vector3(-s, 0, 0)), A.clone().add(new THREE.Vector3(s, 0, 0)));
          seg(A.clone().add(new THREE.Vector3(0, -s, 0)), A.clone().add(new THREE.Vector3(0, s, 0)));
          seg(A.clone().add(new THREE.Vector3(0, 0, -s)), A.clone().add(new THREE.Vector3(0, 0, s)));
          break;
        }
        case 2: {
          seg(A, B);
          const dir = B.clone().sub(A).normalize();
          const side = new THREE.Vector3().crossVectors(dir, new THREE.Vector3(0, 1, 0)).normalize();
          const s = Math.max(8, d.size);
          seg(B, B.clone().addScaledVector(dir, -s).addScaledVector(side, s * 0.6));
          seg(B, B.clone().addScaledVector(dir, -s).addScaledVector(side, -s * 0.6));
          break;
        }
        case 3:
          circle(A, d.size, new THREE.Vector3(1, 0, 0), new THREE.Vector3(0, 0, 1));
          circle(A, d.size, new THREE.Vector3(1, 0, 0), new THREE.Vector3(0, 1, 0));
          circle(A, d.size, new THREE.Vector3(0, 0, 1), new THREE.Vector3(0, 1, 0));
          break;
        case 4:
          circle(A, d.size, new THREE.Vector3(1, 0, 0), new THREE.Vector3(0, 0, 1));
          break;
        case 5: {
          // Cono: origen A, dirección B (unitaria), longitud size, semiángulo extra (rad).
          const dir = B.clone().normalize();
          const up = Math.abs(dir.y) > 0.9 ? new THREE.Vector3(1, 0, 0) : new THREE.Vector3(0, 1, 0);
          const u = new THREE.Vector3().crossVectors(dir, up).normalize();
          const v = new THREE.Vector3().crossVectors(dir, u).normalize();
          const len = d.size;
          const r = Math.tan(Math.min(d.extra, 1.5)) * len;
          const center = A.clone().addScaledVector(dir, len);
          circle(center, r, u, v, 24);
          for (let i = 0; i < 8; i++) {
            const ang = (i / 8) * Math.PI * 2;
            seg(A, center.clone().addScaledVector(u, Math.cos(ang) * r).addScaledVector(v, Math.sin(ang) * r));
          }
          break;
        }
      }
    }
    const geo = this.debug.geometry;
    geo.setAttribute('position', new THREE.Float32BufferAttribute(pos, 3));
    geo.setAttribute('color', new THREE.Float32BufferAttribute(col, 3));
    geo.computeBoundingSphere();
  }

  private labels(): Label[] {
    const tr = this.trace;
    if (!tr) return [];
    const out: Label[] = [];
    const t = this.time;
    for (const e of tr.events) {
      if (e.type === 'bubble' && t >= e.t && t < e.t + 1.8) {
        const obj = this.actors[e.actor];
        if (!obj) continue;
        const p = obj.group.position.clone();
        p.y += obj.meta.class === 'door' ? 270 : 195;
        out.push({ pos: p, text: e.text, color: e.color, kind: 'bubble' });
      }
    }
    for (const d of this.activeDraws()) {
      if (d.kind === 9 && d.text) out.push({ pos: ue(...d.a), text: d.text, color: d.color, kind: 'debug' });
    }
    return out;
  }

  private updateOverlay() {
    const w = this.container.clientWidth;
    const h = this.container.clientHeight;
    const list = this.labels();
    while (this.labelPool.length < list.length) {
      const el = document.createElement('div');
      el.className = 'mir-label';
      this.overlay.appendChild(el);
      this.labelPool.push(el);
    }
    this.labelPool.forEach((el, i) => {
      const l = list[i];
      if (!l) {
        el.style.display = 'none';
        return;
      }
      const p = l.pos.clone().project(this.camera);
      if (p.z > 1) {
        el.style.display = 'none';
        return;
      }
      if (el.textContent !== l.text) el.textContent = l.text;
      const look = l.kind === 'bubble'
        ? `padding:4px 10px;border-radius:999px;font-size:12px;font-weight:600;background:rgba(8,10,14,.85);border:1px solid ${l.color};color:${l.color};box-shadow:0 4px 14px rgba(0,0,0,.4);`
        : `font-size:11px;color:${l.color};text-shadow:0 1px 2px #000;`;
      el.style.cssText = `display:block;left:${((p.x + 1) / 2) * w}px;top:${((1 - p.y) / 2) * h}px;${look}`;
    });

    // Mensajes de GEngine->AddOnScreenDebugMessage.
    const tr = this.trace;
    const t = this.time;
    const msgs = tr ? tr.events.filter((e) => e.type === 'screen' && t >= e.t && t < e.t + e.duration).slice(-8) : [];
    const html = msgs.map((m) => (m.type === 'screen' ? `<div style="color:${m.color}">${escapeHtml(m.text)}</div>` : '')).join('');
    if (this.screenBox.innerHTML !== html) this.screenBox.innerHTML = html;
  }

  private frame() {
    this.controls.update();
    this.apply();
    this.renderer.render(this.scene, this.camera);
    this.updateOverlay();
  }
}

function escapeHtml(s: string) {
  return s.replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]!);
}
