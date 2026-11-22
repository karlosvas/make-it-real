//! Mundo simulado: actores, movimiento tipo CharacterMovement, colisiones sencillas,
//! eventos y dibujos de depuración. Coordenadas y unidades de Unreal (cm, X delante, Z arriba).

use serde::Serialize;
use std::ops::{Add, AddAssign, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl V3 {
    pub const ZERO: V3 = V3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn xy(x: f64, y: f64) -> Self {
        Self { x, y, z: 0.0 }
    }
    pub fn len(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn len2d(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn dot(self, o: V3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn normal(self) -> V3 {
        let l = self.len();
        if l < 1e-8 { V3::ZERO } else { self * (1.0 / l) }
    }
    pub fn normal2d(self) -> V3 {
        V3::xy(self.x, self.y).normal()
    }
    pub fn dist(self, o: V3) -> f64 {
        (self - o).len()
    }
    pub fn dist2d(self, o: V3) -> f64 {
        (self - o).len2d()
    }
    /// Vector unitario hacia donde mira un yaw (grados).
    pub fn from_yaw(yaw_deg: f64) -> V3 {
        let r = yaw_deg.to_radians();
        V3::xy(r.cos(), r.sin())
    }
    pub fn yaw(self) -> f64 {
        self.y.atan2(self.x).to_degrees()
    }
    pub fn rotate_yaw(self, yaw_deg: f64) -> V3 {
        let (s, c) = yaw_deg.to_radians().sin_cos();
        V3::new(self.x * c - self.y * s, self.x * s + self.y * c, self.z)
    }
    pub fn arr(self) -> [f64; 3] {
        [r2(self.x), r2(self.y), r2(self.z)]
    }
}

impl Add for V3 {
    type Output = V3;
    fn add(self, o: V3) -> V3 {
        V3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}
impl AddAssign for V3 {
    fn add_assign(&mut self, o: V3) {
        *self = *self + o;
    }
}
impl Sub for V3 {
    type Output = V3;
    fn sub(self, o: V3) -> V3 {
        V3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}
impl Mul<f64> for V3 {
    type Output = V3;
    fn mul(self, s: f64) -> V3 {
        V3::new(self.x * s, self.y * s, self.z * s)
    }
}
impl Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        V3::new(-self.x, -self.y, -self.z)
    }
}

pub fn r2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

/// Normaliza un ángulo a (-180, 180].
pub fn unwind(mut a: f64) -> f64 {
    if !a.is_finite() {
        return 0.0;
    }
    a %= 360.0;
    if a > 180.0 {
        a -= 360.0;
    }
    if a <= -180.0 {
        a += 360.0;
    }
    a
}

/// Acerca el ángulo `current` a `target` como mucho `step` grados por el camino corto.
pub fn approach_angle(current: f64, target: f64, step: f64) -> f64 {
    let delta = unwind(target - current);
    if delta.abs() <= step { target } else { current + step * delta.signum() }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct Rot {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

impl Rot {
    pub fn yaw(yaw: f64) -> Self {
        Self { pitch: 0.0, yaw, roll: 0.0 }
    }
}

/// Debe coincidir con `EMirClass` en sdk/include/Mir/Glue.h
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    Actor = 0,
    Monigote = 1,
    Door = 2,
    Key = 3,
    Guard = 4,
    TargetPoint = 5,
}

impl Class {
    pub fn is_character(self) -> bool {
        matches!(self, Class::Monigote | Class::Guard)
    }
}

#[derive(Clone, Debug)]
pub struct Actor {
    pub handle: i32,
    pub class: Class,
    pub name: String,
    pub label: Option<String>,
    pub loc: V3,
    pub rot: Rot,
    /// Rotación relativa de los componentes (0 = la hoja de la puerta).
    pub comp_rot: [Rot; 4],
    pub hidden: bool,
    pub destroyed: bool,
    pub input: V3,
    pub max_speed: f64,
    pub orient: bool,
    pub velocity: V3,
    pub radius: f64,
    /// Bandera visual libre (p. ej. el guardia te está viendo).
    pub flag: u8,
    /// Animación de la hoja de la puerta controlada por el mundo (no por el usuario).
    pub door_anim: Option<f64>,
    /// La puerta tiene paredes a los lados.
    pub walls: bool,
    /// Parámetro visual del cono de visión (guardia): (radio, medio ángulo en grados).
    pub sight: Option<(f64, f64)>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    Log { t: f64, category: String, verbosity: String, text: String },
    Screen { t: f64, text: String, color: String, duration: f64 },
    DoorAttempt { t: f64, door: i32, who: i32, allowed: bool },
    Pickup { t: f64, who: i32, key: i32 },
    Choice { t: f64, who: i32, target: i32 },
    Bubble { t: f64, actor: i32, text: String, color: String },
    Crash { t: f64, message: String },
}

#[derive(Clone, Debug, Serialize)]
pub struct Draw {
    pub t: f64,
    pub life: f64,
    pub persistent: bool,
    pub kind: u8,
    pub a: [f64; 3],
    pub b: [f64; 3],
    pub size: f64,
    pub extra: f64,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

pub const MAX_LOGS: usize = 400;
pub const MAX_DRAWS: usize = 6000;

#[derive(Default)]
pub struct World {
    pub actors: Vec<Actor>,
    pub time: f64,
    pub dt: f64,
    pub events: Vec<Event>,
    pub draws: Vec<Draw>,
    pub log_count: usize,
    pub draws_dropped: bool,
    pub fatal_message: Option<String>,
    pub finished_at: Option<f64>,
}

pub fn color_hex(packed_rgba: u32) -> String {
    format!("#{:06x}", packed_rgba >> 8)
}

impl World {
    pub fn new() -> Self {
        Self { dt: 1.0 / 60.0, ..Default::default() }
    }

    pub fn spawn(&mut self, class: Class, name: &str, loc: V3, yaw: f64) -> i32 {
        let handle = self.actors.len() as i32;
        self.actors.push(Actor {
            handle,
            class,
            name: name.to_string(),
            label: None,
            loc,
            rot: Rot::yaw(yaw),
            comp_rot: [Rot::default(); 4],
            hidden: false,
            destroyed: false,
            input: V3::ZERO,
            max_speed: 400.0,
            orient: true,
            velocity: V3::ZERO,
            radius: match class {
                Class::Monigote | Class::Guard => 34.0,
                _ => 0.0,
            },
            flag: 0,
            door_anim: None,
            walls: class == Class::Door,
            sight: None,
        });
        handle
    }

    pub fn actor(&self, h: i32) -> Option<&Actor> {
        if h < 0 { None } else { self.actors.get(h as usize) }
    }
    pub fn actor_mut(&mut self, h: i32) -> Option<&mut Actor> {
        if h < 0 { None } else { self.actors.get_mut(h as usize) }
    }
    pub fn a(&self, h: i32) -> &Actor {
        &self.actors[h as usize]
    }
    pub fn am(&mut self, h: i32) -> &mut Actor {
        &mut self.actors[h as usize]
    }

    pub fn push_event(&mut self, e: Event) {
        self.events.push(e);
    }

    pub fn bubble(&mut self, actor: i32, text: &str, color: &str) {
        let t = self.time;
        self.events.push(Event::Bubble { t, actor, text: text.into(), color: color.into() });
    }

    pub fn log(&mut self, category: &str, verbosity: i32, text: String) {
        if self.log_count >= MAX_LOGS {
            return;
        }
        self.log_count += 1;
        let verbosity = match verbosity {
            1 => "Fatal",
            2 => "Error",
            3 => "Warning",
            4 => "Display",
            5 => "Log",
            6 => "Verbose",
            _ => "VeryVerbose",
        };
        let t = self.time;
        let text = if self.log_count == MAX_LOGS { format!("{text}\n(… demasiados logs, el resto se omite)") } else { text };
        self.events.push(Event::Log { t, category: category.into(), verbosity: verbosity.into(), text });
    }

    pub fn draw(&mut self, d: Draw) {
        if self.draws.len() >= MAX_DRAWS {
            if !self.draws_dropped {
                self.draws_dropped = true;
                self.log("LogMir", 3, "Demasiados DrawDebug*: el resto no se muestra.".into());
            }
            return;
        }
        self.draws.push(d);
    }

    /// Cajas que bloquean el paso: (centro, medio ancho X, medio ancho Y, yaw).
    fn blockers(&self) -> Vec<(V3, f64, f64, f64)> {
        let mut out = Vec::new();
        for a in &self.actors {
            if a.class != Class::Door || a.destroyed {
                continue;
            }
            let yaw = a.rot.yaw;
            if a.comp_rot[0].yaw.abs() < 25.0 {
                out.push((a.loc, 6.0, 60.0, yaw));
            }
            if a.walls {
                for side in [-1.0, 1.0] {
                    let c = a.loc + V3::xy(0.0, side * 235.0).rotate_yaw(yaw);
                    out.push((c, 10.0, 175.0, yaw));
                }
            }
        }
        out
    }

    /// Mueve a los personajes según el input acumulado este frame (como CharacterMovement).
    pub fn integrate(&mut self) {
        let dt = self.dt;
        let blockers = self.blockers();
        for a in self.actors.iter_mut() {
            if a.class.is_character() && !a.destroyed {
                let mut input = a.input;
                input.z = 0.0;
                let l = input.len();
                if l > 1.0 {
                    input = input * (1.0 / l);
                }
                a.velocity = input * a.max_speed;
                if a.velocity.len2d() > 0.0 {
                    let before = a.loc;
                    a.loc += a.velocity * dt;
                    for _ in 0..2 {
                        for &(c, hx, hy, yaw) in &blockers {
                            a.loc = push_out(a.loc, a.radius, c, hx, hy, yaw);
                        }
                    }
                    a.velocity = (a.loc - before) * (1.0 / dt);
                    if a.orient && input.len2d() > 0.05 {
                        a.rot.yaw = unwind(approach_angle(a.rot.yaw, input.yaw(), 720.0 * dt));
                    }
                }
                a.input = V3::ZERO;
            }
            if let Some(target) = a.door_anim {
                a.comp_rot[0].yaw = approach_angle(a.comp_rot[0].yaw, target, 220.0 * dt);
            }
        }
    }

    /// Movimiento "de guion" (personajes que no programa el usuario): anda hacia `target`.
    /// Devuelve true al llegar.
    pub fn walk_to(&mut self, h: i32, target: V3, speed: f64) -> bool {
        let dt = self.dt;
        let a = self.am(h);
        let to = V3::xy(target.x - a.loc.x, target.y - a.loc.y);
        let d = to.len2d();
        if d < 4.0 {
            return true;
        }
        a.max_speed = speed;
        let step = speed * dt;
        a.input = if d < step { to * (1.0 / step) } else { to.normal() };
        false
    }

    /// Gira poco a poco hacia un punto. Devuelve true cuando ya mira hacia él.
    pub fn turn_towards(&mut self, h: i32, target: V3, deg_per_sec: f64) -> bool {
        let dt = self.dt;
        let a = self.am(h);
        let want = (target - a.loc).yaw();
        a.rot.yaw = unwind(approach_angle(a.rot.yaw, want, deg_per_sec * dt));
        unwind(a.rot.yaw - want).abs() < 0.5
    }
}

fn push_out(p: V3, r: f64, c: V3, hx: f64, hy: f64, yaw: f64) -> V3 {
    let local = (p - c).rotate_yaw(-yaw);
    let cx = local.x.clamp(-hx, hx);
    let cy = local.y.clamp(-hy, hy);
    let d = V3::xy(local.x - cx, local.y - cy);
    let dist = d.len2d();
    if dist >= r {
        return p;
    }
    let push = if dist > 1e-6 {
        d * ((r - dist) / dist)
    } else {
        // El centro está dentro de la caja: sacarlo por el lado más cercano.
        let px = hx - local.x.abs() + r;
        let py = hy - local.y.abs() + r;
        if px < py { V3::xy(px * local.x.signum().max(0.0).mul_add(2.0, -1.0), 0.0) } else { V3::xy(0.0, py * local.y.signum().max(0.0).mul_add(2.0, -1.0)) }
    };
    p + push.rotate_yaw(yaw)
}
