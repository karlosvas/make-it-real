//! 07 · La llave más cercana · 08 · Patrulla · 09 · ¿Te ve el guardia? · 10 · La puerta se cierra sola

use super::Builder;
use super::common::*;
use crate::sim::runner::{Ctx, Record, Scenario};
use crate::sim::world::{Class, Event, V3, World, unwind};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// ---------- 07 ----------

pub fn scenarios_07() -> Vec<(&'static str, Builder)> {
    vec![("tres-llaves", e7_tres), ("la-ultima", e7_ultima), ("una-llave", e7_una), ("sin-llaves", e7_sin)]
}

fn ex07(keys: &[(&str, V3)]) -> Scenario {
    let mut w = World::new();
    let moni = w.spawn(Class::Monigote, "Monigote", V3::ZERO, 0.0);
    let mut wires = Vec::new();
    let mut handles = Vec::new();
    for (label, at) in keys {
        let k = w.spawn(Class::Key, &format!("Llave {label}"), *at, 0.0);
        w.am(k).label = Some(format!("Llave {label}"));
        wires.push((WIRE_KEY, moni, k));
        handles.push((k, at.len2d()));
    }
    let expected = handles.iter().min_by(|a, b| a.1.total_cmp(&b.1)).map(|(h, _)| *h).unwrap_or(-1);
    let mut phase = 0;
    let mut target = -1;
    Scenario {
        duration: 7.0,
        world: w,
        wires,
        script: Box::new(move |ctx: &mut Ctx| {
            let t = ctx.t();
            match phase {
                0 if t >= 0.5 => {
                    target = ctx.call(FN_FIND_CLOSEST_KEY, moni, 0)?;
                    let w = ctx.w();
                    w.push_event(Event::Choice { t, who: moni, target });
                    let text = w.actor(target).map(|a| format!("¡A por la {}!", a.name)).unwrap_or_else(|| "nullptr: no hay llaves".into());
                    w.bubble(moni, &text, if target >= 0 { YELLOW } else { BLUE });
                    if w.actor(target).is_some_and(|a| a.class == Class::Key) {
                        phase = 1;
                    } else {
                        phase = 9;
                        ctx.finish();
                    }
                }
                1 => {
                    let at = ctx.w().a(target).loc;
                    if ctx.w().walk_to(moni, at, WALK_SPEED) {
                        phase = 9;
                        ctx.finish();
                    }
                }
                _ => {}
            }
            Ok(())
        }),
        check: Box::new(move |r: &Record| {
            let Some(got) = r.choice() else { return Err("No se llegó a llamar a FindClosestKey.".into()) };
            let name = |h: i32| if h < 0 { "nullptr".to_string() } else { r.world.actor(h).map(|a| a.name.clone()).unwrap_or(format!("un puntero raro ({h})")) };
            if got == expected {
                if expected < 0 {
                    Ok("No hay llaves y devuelve nullptr.".into())
                } else {
                    Ok(format!("Elige la {}: la más cercana.", name(expected)))
                }
            } else if expected < 0 {
                Err(format!("No hay llaves: tenía que devolver nullptr, pero ha devuelto {}.", name(got)))
            } else {
                Err(format!("Ha devuelto {}, pero la más cercana era la {}.", name(got), name(expected)))
            }
        }),
    }
}

fn e7_tres() -> Scenario {
    ex07(&[("A", V3::xy(420.0, 260.0)), ("B", V3::xy(-260.0, 180.0)), ("C", V3::xy(150.0, -330.0))])
}
fn e7_ultima() -> Scenario {
    ex07(&[("A", V3::xy(600.0, 0.0)), ("B", V3::xy(0.0, 520.0)), ("C", V3::xy(-200.0, -150.0))])
}
fn e7_una() -> Scenario {
    ex07(&[("A", V3::xy(-480.0, 330.0))])
}
fn e7_sin() -> Scenario {
    ex07(&[])
}

// ---------- 08 ----------

pub fn scenarios_08() -> Vec<(&'static str, Builder)> {
    vec![("cuadrado", e8_cuadrado), ("triangulo", e8_triangulo), ("sin-puntos", e8_sin)]
}

fn ex08(start: V3, points: &[V3], duration: f64) -> Scenario {
    let mut w = World::new();
    let first = points.first().copied().unwrap_or(V3::xy(100.0, 0.0));
    let guard = w.spawn(Class::Guard, "Guardia", start, yaw_towards(start, first));
    let mut wires = Vec::new();
    let mut wps = Vec::new();
    for (i, p) in points.iter().enumerate() {
        let h = w.spawn(Class::TargetPoint, &format!("Waypoint {i}"), *p, 0.0);
        w.am(h).label = Some(format!("{i}"));
        wires.push((WIRE_WAYPOINT, guard, h));
        wps.push(*p);
    }
    let n = wps.len();
    Scenario {
        duration,
        world: w,
        wires,
        script: Box::new(|_| Ok(())),
        check: Box::new(move |r: &Record| {
            let moved = r.max_displacement(guard);
            if n == 0 {
                return if moved < 5.0 { Ok("Sin waypoints se queda quieto (y no revienta).".into()) } else { Err("Sin waypoints debería quedarse quieto.".into()) };
            }
            if moved < 10.0 {
                return Err("El guardia no se mueve. ¿Llamas a AddMovementInput hacia Waypoints[CurrentIndex]?".into());
            }
            let mut visits: Vec<usize> = Vec::new();
            for f in &r.frames {
                let p = f.actors[guard as usize].loc;
                if let Some((i, _)) = wps.iter().enumerate().map(|(i, w)| (i, p.dist2d(*w))).filter(|(_, d)| *d < 62.0).min_by(|a, b| a.1.total_cmp(&b.1)) {
                    if visits.last() != Some(&i) {
                        visits.push(i);
                    }
                }
            }
            let expected: Vec<usize> = (0..n).chain(std::iter::once(0)).collect();
            let shown = |v: &[usize]| v.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" → ");
            if visits.len() >= expected.len() && visits[..expected.len()] == expected[..] {
                Ok(format!("Recorre {} y vuelve a empezar.", shown(&expected)))
            } else if visits.len() < expected.len() && expected.starts_with(&visits) {
                if visits.len() == n {
                    Err(format!("Ha hecho {} pero después del último no vuelve al 0.", shown(&visits)))
                } else {
                    Err(format!("Solo ha llegado a recorrer {} (se esperaba {}).", if visits.is_empty() { "nada".into() } else { shown(&visits) }, shown(&expected)))
                }
            } else {
                Err(format!("Orden incorrecto: ha hecho {} y se esperaba {}.", shown(&visits), shown(&expected)))
            }
        }),
    }
}

fn e8_cuadrado() -> Scenario {
    ex08(V3::xy(0.0, 0.0), &[V3::xy(0.0, 0.0), V3::xy(600.0, 0.0), V3::xy(600.0, 600.0), V3::xy(0.0, 600.0)], 10.0)
}
fn e8_triangulo() -> Scenario {
    ex08(V3::xy(-300.0, -250.0), &[V3::xy(300.0, -300.0), V3::xy(300.0, 300.0), V3::xy(-300.0, 0.0)], 10.5)
}
fn e8_sin() -> Scenario {
    ex08(V3::xy(0.0, 0.0), &[], 2.0)
}

// ---------- 09 ----------

pub fn scenarios_09() -> Vec<(&'static str, Builder)> {
    vec![("cruza-por-delante", e9_cruza), ("se-acerca-de-frente", e9_frente), ("por-detras", e9_detras), ("por-el-lado", e9_lado)]
}

const SIGHT: f64 = 800.0;
const HALF: f64 = 45.0;

struct Sample {
    t: f64,
    dist: f64,
    angle: f64,
    seen: bool,
}

fn ex09(from: V3, to: V3) -> Scenario {
    let mut w = World::new();
    let guard = w.spawn(Class::Guard, "Guardia", V3::ZERO, 0.0);
    w.am(guard).sight = Some((SIGHT, HALF));
    let moni = w.spawn(Class::Monigote, "Monigote", from, yaw_towards(from, to));
    let duration = from.dist2d(to) / WALK_SPEED + 0.8;
    let samples = Rc::new(RefCell::new(Vec::<Sample>::new()));
    let rec = samples.clone();
    let mut done = false;
    Scenario {
        duration,
        world: w,
        wires: vec![],
        script: Box::new(move |ctx: &mut Ctx| {
            let seen = ctx.call(FN_CAN_SEE, guard, moni)? != 0;
            let t = ctx.t();
            let w = ctx.w();
            let g = w.a(guard);
            let to_m = w.a(moni).loc - g.loc;
            let angle = unwind(to_m.yaw() - g.rot.yaw).abs();
            rec.borrow_mut().push(Sample { t, dist: to_m.len2d(), angle, seen });
            w.am(guard).flag = seen as u8;
            if !done && w.walk_to(moni, to, WALK_SPEED) {
                done = true;
            }
            Ok(())
        }),
        check: Box::new(move |_r: &Record| {
            let s = samples.borrow();
            let mut checked = 0;
            let mut seen_frames = 0;
            for x in s.iter() {
                if (x.dist - SIGHT).abs() < 8.0 || (x.angle - HALF).abs() < 1.5 {
                    continue;
                }
                checked += 1;
                let truth = x.dist <= SIGHT && x.angle <= HALF;
                if truth {
                    seen_frames += 1;
                }
                if truth != x.seen {
                    return Err(format!(
                        "A los {:.1} s el monigote estaba a {} y {:.0}° del frente del guardia: {}",
                        x.t,
                        cm(x.dist),
                        x.angle,
                        if truth { "debería verlo, pero CanSee devolvió false." } else { "no debería verlo, pero CanSee devolvió true." }
                    ));
                }
            }
            if checked == 0 {
                return Err("No se llegó a llamar a CanSee.".into());
            }
            Ok(if seen_frames > 0 {
                format!("Acierta en los {checked} frames (lo ve en {seen_frames}).")
            } else {
                format!("Acierta en los {checked} frames (nunca lo ve, como debe).")
            })
        }),
    }
}

fn e9_cruza() -> Scenario {
    ex09(V3::xy(450.0, -650.0), V3::xy(450.0, 650.0))
}
fn e9_frente() -> Scenario {
    ex09(V3::xy(1300.0, 80.0), V3::xy(250.0, 0.0))
}
fn e9_detras() -> Scenario {
    ex09(V3::xy(-300.0, -450.0), V3::xy(-300.0, 450.0))
}
fn e9_lado() -> Scenario {
    ex09(V3::xy(100.0, 420.0), V3::xy(760.0, 420.0))
}

// ---------- 10 ----------

pub fn scenarios_10() -> Vec<(&'static str, Builder)> {
    vec![("se-cierra-sola", e10_sola), ("reabrir", e10_reabrir)]
}

fn ex10(reopen_after: Option<f64>) -> Scenario {
    let mut w = World::new();
    let door = w.spawn(Class::Door, "Puerta", V3::ZERO, 0.0);
    let moni = w.spawn(Class::Monigote, "Monigote", V3::xy(480.0, -160.0), 160.0);
    let t_open = Rc::new(Cell::new(f64::NAN));
    let so = t_open.clone();
    let mut phase = 0;
    let mut wait = 0.0;
    let expected_delay = 3.0 + reopen_after.unwrap_or(0.0);
    Scenario {
        duration: 11.0,
        world: w,
        wires: vec![],
        script: Box::new(move |ctx: &mut Ctx| {
            let t = ctx.t();
            match phase {
                0 => {
                    if ctx.w().walk_to(moni, V3::xy(160.0, 0.0), WALK_SPEED) {
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
                    ctx.w().bubble(moni, "¡Abro!", BLUE);
                    phase = 3;
                }
                3 => {
                    if let Some(after) = reopen_after {
                        if t >= so.get() + after {
                            ctx.call(FN_OPEN, door, 0)?;
                            ctx.w().bubble(moni, "¡Otra vez!", BLUE);
                            phase = 4;
                        }
                    } else {
                        phase = 4;
                    }
                }
                4 if t >= so.get() + expected_delay + 1.6 => {
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
            let leaf = |f: &crate::sim::runner::Frame| f.actors[door as usize].leaf.abs();
            let Some(open_at) = r.frames.iter().find(|f| f.t >= opened && leaf(f) > 45.0).map(|f| f.t) else {
                return Err("Open() no abre la puerta.".into());
            };
            if open_at > opened + 0.6 {
                return Err("La puerta tarda demasiado en abrirse.".into());
            }
            let close_at = r.frames.iter().find(|f| f.t > open_at && leaf(f) < 45.0).map(|f| f.t);
            let lo = opened + expected_delay - 0.2;
            let hi = opened + expected_delay + 0.5;
            match close_at {
                None => Err(format!("La puerta no se cierra sola: tendría que cerrarse a los {expected_delay:.0} s.")),
                Some(c) if c < lo => {
                    let after = c - opened;
                    if reopen_after.is_some() && (after - 3.0).abs() < 0.6 {
                        Err(format!("Se ha cerrado a los {after:.1} s del primer Open(), pero al volver a abrirla la cuenta tendría que empezar de nuevo (cerrarse a los {expected_delay:.0} s)."))
                    } else {
                        Err(format!("Se cierra demasiado pronto: a los {after:.1} s (se esperaba a los {expected_delay:.0} s)."))
                    }
                }
                Some(c) if c > hi => Err(format!("Se cierra demasiado tarde: a los {:.1} s (se esperaba a los {expected_delay:.0} s).", c - opened)),
                Some(c) => Ok(format!("Se cierra sola a los {:.1} s.", c - opened)),
            }
        }),
    }
}

fn e10_sola() -> Scenario {
    ex10(None)
}
fn e10_reabrir() -> Scenario {
    ex10(Some(2.0))
}
