//! 01 · Ve hacia la puerta y 02 · Para a un metro — Tick + AddMovementInput.

use super::Builder;
use super::common::*;
use crate::sim::runner::{Ctx, Record, Scenario};
use crate::sim::world::{Class, V3, World};

struct Setup {
    world: World,
    moni: i32,
    door: i32,
}

fn setup(door_at: V3) -> Setup {
    let mut w = World::new();
    let moni = w.spawn(Class::Monigote, "Monigote", V3::ZERO, 0.0);
    let door = w.spawn(Class::Door, "Puerta", door_at, yaw_towards(door_at, V3::ZERO));
    w.am(door).walls = false;
    Setup { world: w, moni, door }
}

// ---------- 01 ----------

pub fn scenarios_01() -> Vec<(&'static str, Builder)> {
    vec![("delante", e1_delante), ("lejos-en-diagonal", e1_lejos), ("detras", e1_detras)]
}

fn ex01(door_at: V3) -> Scenario {
    let s = setup(door_at);
    let (moni, door) = (s.moni, s.door);
    Scenario {
        duration: 6.0,
        world: s.world,
        wires: vec![(WIRE_TARGET_DOOR, moni, door)],
        script: Box::new(move |ctx: &mut Ctx| {
            let w = ctx.w();
            if w.a(moni).loc.dist2d(w.a(door).loc) < 110.0 {
                ctx.finish();
            }
            Ok(())
        }),
        check: Box::new(move |r: &Record| {
            let arrived = r.first_time(|f| f.actors[moni as usize].loc.dist2d(f.actors[door as usize].loc) <= 115.0);
            match arrived {
                Some(t) => Ok(format!("Llega a la puerta en {t:.1} s.")),
                None if r.max_displacement(moni) < 10.0 => Err("El monigote no se ha movido. ¿Has llamado a AddMovementInput en el Tick?".into()),
                None => {
                    let last = r.last();
                    let d = last.actors[moni as usize].loc.dist2d(last.actors[door as usize].loc);
                    Err(format!("No llega a la puerta: se queda a {} de ella. ¿La dirección es Puerta - Monigote?", cm(d)))
                }
            }
        }),
    }
}

fn e1_delante() -> Scenario {
    ex01(V3::xy(650.0, 0.0))
}
fn e1_lejos() -> Scenario {
    ex01(V3::xy(800.0, 520.0))
}
fn e1_detras() -> Scenario {
    ex01(V3::xy(-520.0, -380.0))
}

// ---------- 02 ----------

pub fn scenarios_02() -> Vec<(&'static str, Builder)> {
    vec![("delante", e2_delante), ("en-diagonal", e2_diagonal), ("ya-esta-cerca", e2_cerca)]
}

fn ex02(door_at: V3, already_close: bool) -> Scenario {
    let s = setup(door_at);
    let (moni, door) = (s.moni, s.door);
    Scenario {
        duration: 4.0,
        world: s.world,
        wires: vec![(WIRE_TARGET_DOOR, moni, door)],
        script: Box::new(|_| Ok(())),
        check: Box::new(move |r: &Record| {
            let min = r.min_dist2d(moni, door);
            let last = r.last();
            let d = last.actors[moni as usize].loc.dist2d(last.actors[door as usize].loc);
            let speed = last.actors[moni as usize].speed;
            if already_close {
                if r.max_displacement(moni) > 10.0 {
                    return Err(format!("Empezaba a menos de 1 m y aun así se ha movido (ha acabado a {}).", cm(d)));
                }
                return Ok("Ya estaba a menos de 1 m y no se ha movido.".into());
            }
            if r.max_displacement(moni) < 10.0 {
                return Err("El monigote no se ha movido. ¿Te has cargado el AddMovementInput?".into());
            }
            if min < 75.0 {
                return Err(format!("Se ha chocado con la puerta (llegó a {} de ella).", cm(min)));
            }
            if speed > 5.0 {
                return Err("Al final sigue moviéndose: tiene que quedarse quieto.".into());
            }
            if !(85.0..=110.0).contains(&d) {
                return Err(format!("Se ha parado a {}: tiene que quedarse a ~1 m (100 cm).", cm(d)));
            }
            Ok(format!("Se para a {} de la puerta.", cm(d)))
        }),
    }
}

fn e2_delante() -> Scenario {
    ex02(V3::xy(600.0, 0.0), false)
}
fn e2_diagonal() -> Scenario {
    ex02(V3::xy(450.0, -380.0), false)
}
fn e2_cerca() -> Scenario {
    ex02(V3::xy(80.0, 0.0), true)
}
