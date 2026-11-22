//! Piezas compartidas por los escenarios de los ejercicios.

use crate::sim::runner::{CheckFn, Ctx, ScriptFn};
use crate::sim::world::{V3, unwind};

// Ids de funciones de mir_call (deben coincidir con cada glue.cpp).
pub const FN_CAN_OPEN: i32 = 1;
pub const FN_FIND_CLOSEST_KEY: i32 = 2;
pub const FN_CAN_SEE: i32 = 3;
pub const FN_OPEN: i32 = 4;
pub const FN_CLOSE: i32 = 5;

// Slots de mir_wire (asignaciones del "panel Details").
pub const WIRE_TARGET_DOOR: i32 = 1;
pub const WIRE_KEY: i32 = 2;
pub const WIRE_WAYPOINT: i32 = 3;

pub const GREEN: &str = "#4ade80";
pub const RED: &str = "#f87171";
pub const YELLOW: &str = "#facc15";
pub const BLUE: &str = "#60a5fa";

pub const WALK_SPEED: f64 = 260.0;

pub fn yaw_towards(from: V3, to: V3) -> f64 {
    (to - from).yaw()
}

/// Gira a un yaw concreto poco a poco. Devuelve true al terminar.
pub fn turn_to_yaw(ctx: &mut Ctx, h: i32, yaw: f64, rate: f64) -> bool {
    let w = ctx.w();
    let dt = w.dt;
    let a = w.am(h);
    a.rot.yaw = unwind(crate::sim::world::approach_angle(a.rot.yaw, yaw, rate * dt));
    unwind(a.rot.yaw - yaw).abs() < 0.5
}

#[derive(Clone, Copy)]
pub enum Facing {
    /// Se gira para mirar a la puerta.
    Door,
    /// Se gira a un yaw concreto (grados).
    Yaw(f64),
    /// Llega andando de espaldas y no se gira.
    Backwards,
}

/// Guion: el monigote recorre `path`, llega a `approach`, se orienta y llama a CanOpen.
/// Si la puerta le deja, la cruza hasta `through`.
pub struct DoorAttempt {
    pub moni: i32,
    pub door: i32,
    pub path: Vec<V3>,
    pub approach: V3,
    pub facing: Facing,
    pub through: V3,
}

pub fn door_attempt_script(cfg: DoorAttempt) -> ScriptFn {
    let mut phase = 0;
    let mut idx = 0;
    let mut wait_until = 0.0;
    Box::new(move |ctx: &mut Ctx| {
        let t = ctx.t();
        match phase {
            0 => {
                if matches!(cfg.facing, Facing::Backwards) {
                    ctx.w().am(cfg.moni).orient = false;
                }
                let target = cfg.path.get(idx).copied().unwrap_or(cfg.approach);
                if ctx.w().walk_to(cfg.moni, target, WALK_SPEED) {
                    if idx < cfg.path.len() {
                        idx += 1;
                    } else {
                        phase = 1;
                    }
                }
            }
            1 => {
                let done = match cfg.facing {
                    Facing::Door => {
                        let d = ctx.w().a(cfg.door).loc;
                        ctx.w().turn_towards(cfg.moni, d, 300.0)
                    }
                    Facing::Yaw(y) => turn_to_yaw(ctx, cfg.moni, y, 300.0),
                    Facing::Backwards => true,
                };
                if done {
                    phase = 2;
                    wait_until = t + 0.35;
                }
            }
            2 if t >= wait_until => {
                let allowed = ctx.call(FN_CAN_OPEN, cfg.door, cfg.moni)? != 0;
                let w = ctx.w();
                w.push_event(crate::sim::world::Event::DoorAttempt { t, door: cfg.door, who: cfg.moni, allowed });
                if allowed {
                    let door = w.a(cfg.door);
                    let front = (w.a(cfg.moni).loc - door.loc).dot(V3::from_yaw(door.rot.yaw)) > 0.0;
                    w.am(cfg.door).door_anim = Some(if front { -100.0 } else { 100.0 });
                    w.bubble(cfg.door, "Abierta", GREEN);
                    phase = 3;
                    wait_until = t + 0.55;
                } else {
                    w.bubble(cfg.door, "No se abre", RED);
                    phase = 9;
                    ctx.finish();
                }
            }
            3 if t >= wait_until => {
                ctx.w().am(cfg.moni).orient = true;
                let center = ctx.w().a(cfg.door).loc;
                if ctx.w().walk_to(cfg.moni, center, WALK_SPEED) {
                    phase = 4;
                }
            }
            4 => {
                if ctx.w().walk_to(cfg.moni, cfg.through, WALK_SPEED) {
                    phase = 9;
                    ctx.finish();
                }
            }
            _ => {}
        }
        Ok(())
    })
}

/// Comprueba el resultado del intento de abrir la puerta.
pub fn expect_door(expected: bool, _situation: &'static str) -> CheckFn {
    Box::new(move |r| match r.door_attempt() {
        None => Err("El monigote no llegó a intentar abrir la puerta.".into()),
        Some((_, allowed)) if allowed == expected => Ok(if expected { "La puerta se abre, como debe.".into() } else { "La puerta no se abre, como debe.".into() }),
        Some((_, allowed)) => Err(if allowed { "La puerta se ha abierto, pero no debería.".into() } else { "La puerta no se ha abierto, pero debería.".into() }),
    })
}

pub fn cm(v: f64) -> String {
    format!("{:.0} cm", v)
}
