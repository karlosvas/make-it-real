//! 04 · Y además, mirándola · 05 · Ábrete con suavidad · 06 · Sin llave no entras

use super::Builder;
use super::common::*;
use crate::sim::runner::{Ctx, Record, Scenario};
use crate::sim::world::{Class, V3, World};
use std::cell::Cell;
use std::rc::Rc;

// ---------- 04 ----------

pub fn scenarios_04() -> Vec<(&'static str, Builder)> {
    vec![
        ("mirando", e4_mirando),
        ("de-espaldas", e4_espaldas),
        ("mirando-de-lado", e4_lado),
        ("casi-de-frente", e4_casi),
        ("detras-mirando", e4_detras),
    ]
}

fn ex04(start: V3, start_yaw: f64, approach: V3, facing: Facing, expected: bool, situation: &'static str) -> Scenario {
    let mut w = World::new();
    let door = w.spawn(Class::Door, "Puerta", V3::ZERO, 0.0);
    let moni = w.spawn(Class::Monigote, "Monigote", start, start_yaw);
    let through = if approach.x > 0.0 { V3::xy(-260.0, 0.0) } else { V3::xy(260.0, 0.0) };
    Scenario {
        duration: 10.0,
        world: w,
        wires: vec![],
        script: door_attempt_script(DoorAttempt { moni, door, path: vec![], approach, facing, through }),
        check: expect_door(expected, situation),
    }
}

fn e4_mirando() -> Scenario {
    ex04(V3::xy(500.0, 220.0), 200.0, V3::xy(120.0, 0.0), Facing::Door, true, "Delante y mirando")
}
fn e4_espaldas() -> Scenario {
    ex04(V3::xy(520.0, 0.0), 0.0, V3::xy(130.0, 0.0), Facing::Backwards, false, "Delante, de espaldas")
}
fn e4_lado() -> Scenario {
    ex04(V3::xy(500.0, -220.0), 160.0, V3::xy(120.0, 0.0), Facing::Yaw(110.0), false, "Delante, girado 70°")
}
fn e4_casi() -> Scenario {
    ex04(V3::xy(500.0, 220.0), 200.0, V3::xy(120.0, 0.0), Facing::Yaw(-150.0), true, "Delante, girado 30°")
}
fn e4_detras() -> Scenario {
    ex04(V3::xy(-500.0, -220.0), 20.0, V3::xy(-120.0, 0.0), Facing::Door, false, "Detrás y mirando")
}

// ---------- 05 ----------

pub fn scenarios_05() -> Vec<(&'static str, Builder)> {
    vec![("abrir", e5_abrir), ("abrir-y-cerrar", e5_abrir_cerrar)]
}

const MAX_STEP_DEG: f64 = 10.0;

fn ex05(close_after: Option<f64>) -> Scenario {
    let mut w = World::new();
    let door = w.spawn(Class::Door, "Puerta", V3::ZERO, 0.0);
    let moni = w.spawn(Class::Monigote, "Monigote", V3::xy(480.0, 160.0), 200.0);
    let t_open = Rc::new(Cell::new(f64::NAN));
    let t_close = Rc::new(Cell::new(f64::NAN));
    let (so, sc) = (t_open.clone(), t_close.clone());
    let mut phase = 0;
    let mut wait = 0.0;
    Scenario {
        duration: 9.5,
        world: w,
        wires: vec![],
        script: Box::new(move |ctx: &mut Ctx| {
            let t = ctx.t();
            match phase {
                0 => {
                    if ctx.w().walk_to(moni, V3::xy(170.0, 0.0), WALK_SPEED) {
                        phase = 1;
                    }
                }
                1 => {
                    if ctx.w().turn_towards(moni, V3::ZERO, 300.0) {
                        phase = 2;
                        wait = t + 0.3;
                    }
                }
                2 if t >= wait => {
                    ctx.call(FN_OPEN, door, 0)?;
                    so.set(t);
                    ctx.w().bubble(moni, "¡Ábrete!", BLUE);
                    phase = 3;
                }
                3 => {
                    if let Some(after) = close_after {
                        if t >= so.get() + after {
                            ctx.call(FN_CLOSE, door, 0)?;
                            sc.set(t);
                            ctx.w().bubble(moni, "¡Ciérrate!", BLUE);
                            phase = 4;
                        }
                    } else if t >= so.get() + 3.6 {
                        ctx.finish();
                        phase = 9;
                    }
                }
                4 if t >= sc.get() + 3.6 => {
                    ctx.finish();
                    phase = 9;
                }
                _ => {}
            }
            Ok(())
        }),
        check: Box::new(move |r: &Record| {
            let opened = t_open.get();
            if opened.is_nan() {
                return Err("El monigote no llegó a abrir la puerta.".into());
            }
            let leaf = |i: usize| r.frames[i].actors[door as usize].leaf;
            // Sin saltos bruscos.
            for i in 1..r.frames.len() {
                let step = (leaf(i) - leaf(i - 1)).abs();
                if step > MAX_STEP_DEG {
                    return Err(format!(
                        "La puerta gira {step:.0}° en un solo frame (a los {:.2} s): demasiado brusco, el máximo es {MAX_STEP_DEG:.0}°. Interpola con FInterpTo.",
                        r.frames[i].t
                    ));
                }
            }
            let at = |time: f64| r.frames.iter().find(|f| f.t >= time).map(|f| f.actors[door as usize].leaf.abs());
            let before = r.frames.iter().filter(|f| f.t < opened).map(|f| f.actors[door as usize].leaf.abs()).fold(0.0, f64::max);
            if before > 1.0 {
                return Err("La puerta se mueve antes de que nadie la abra.".into());
            }
            let close_t = t_close.get();
            let open_check = if close_t.is_nan() { opened + 3.0 } else { close_t - 0.05 };
            match at(open_check) {
                Some(y) if y >= 80.0 && y <= 100.0 => {}
                Some(y) if close_t.is_nan() => return Err(format!("A los 3 s de Open() la puerta está a {y:.0}°: debería estar casi en 90°.")),
                Some(y) => return Err(format!("Antes de cerrar, la puerta solo había llegado a {y:.0}°.")),
                None => return Err("No dio tiempo a comprobar la apertura.".into()),
            }
            if !close_t.is_nan() {
                match at(close_t + 3.0) {
                    Some(y) if y <= 3.0 => return Ok("Se abre y se cierra con suavidad.".into()),
                    Some(y) => return Err(format!("A los 3 s de Close() la puerta sigue a {y:.0}°: debería volver a 0°.")),
                    None => return Err("No dio tiempo a comprobar el cierre.".into()),
                }
            }
            Ok("Se abre con suavidad hasta 90°.".into())
        }),
    }
}

fn e5_abrir() -> Scenario {
    ex05(None)
}
fn e5_abrir_cerrar() -> Scenario {
    ex05(Some(2.5))
}

// ---------- 06 ----------

pub fn scenarios_06() -> Vec<(&'static str, Builder)> {
    vec![("sin-llave", e6_sin), ("con-llave", e6_con), ("con-llave-por-detras", e6_detras)]
}

fn ex06(start: V3, key_at: V3, take_key: bool, approach: V3, expected: bool, situation: &'static str) -> Scenario {
    let mut w = World::new();
    let door = w.spawn(Class::Door, "Puerta", V3::ZERO, 0.0);
    let path = if take_key { vec![key_at] } else { vec![] };
    let first = path.first().copied().unwrap_or(approach);
    let moni = w.spawn(Class::Monigote, "Monigote", start, yaw_towards(start, first));
    let key = w.spawn(Class::Key, "Llave", key_at, 0.0);
    w.am(key).label = Some("Llave".into());
    let through = if approach.x > 0.0 { V3::xy(-260.0, 0.0) } else { V3::xy(260.0, 0.0) };
    Scenario {
        duration: 12.0,
        world: w,
        wires: vec![],
        script: door_attempt_script(DoorAttempt { moni, door, path, approach, facing: Facing::Door, through }),
        check: expect_door(expected, situation),
    }
}

fn e6_sin() -> Scenario {
    ex06(V3::xy(520.0, -260.0), V3::xy(380.0, 360.0), false, V3::xy(120.0, 0.0), false, "Delante pero sin llave")
}
fn e6_con() -> Scenario {
    ex06(V3::xy(560.0, 320.0), V3::xy(330.0, 250.0), true, V3::xy(120.0, 0.0), true, "Delante y con llave")
}
fn e6_detras() -> Scenario {
    ex06(V3::xy(-560.0, -320.0), V3::xy(-330.0, -250.0), true, V3::xy(-120.0, 0.0), false, "Con llave pero por detrás")
}
