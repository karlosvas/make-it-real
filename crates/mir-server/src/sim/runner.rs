//! Ejecuta un escenario frame a frame y graba la traza que reproduce el visor.

use super::host::{Compiled, Sim};
use super::world::{Class, Draw, Event, V3, World, r2};
use serde::Serialize;

pub type ScriptFn = Box<dyn FnMut(&mut Ctx) -> Result<(), String>>;
pub type CheckFn = Box<dyn Fn(&Record) -> Result<String, String>>;

/// Id de función genérica que el mundo llama al recoger una llave (ver glue.cpp).
pub const FN_ON_KEY_PICKED: i32 = 10;

pub struct Scenario {
    pub duration: f64,
    pub world: World,
    /// (slot, a, b): asignaciones tipo "panel Details" antes de BeginPlay.
    pub wires: Vec<(i32, i32, i32)>,
    pub script: ScriptFn,
    pub check: CheckFn,
}

pub struct Ctx<'a> {
    pub sim: &'a mut Sim,
}

impl Ctx<'_> {
    pub fn w(&mut self) -> &mut World {
        self.sim.world_mut()
    }
    pub fn t(&self) -> f64 {
        self.sim.world().time
    }
    pub fn call(&mut self, function: i32, a: i32, b: i32) -> Result<i32, String> {
        self.sim.call(function, a, b)
    }
    pub fn finish(&mut self) {
        let w = self.w();
        if w.finished_at.is_none() {
            w.finished_at = Some(w.time);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ActorFrame {
    pub loc: V3,
    pub yaw: f64,
    pub leaf: f64,
    pub hidden: bool,
    pub flag: u8,
    pub speed: f64,
}

pub struct Frame {
    pub t: f64,
    pub actors: Vec<ActorFrame>,
}

pub struct Record {
    pub frames: Vec<Frame>,
    pub events: Vec<Event>,
    pub crash: Option<String>,
    pub world: World,
}

impl Record {
    pub fn last(&self) -> &Frame {
        self.frames.last().expect("al menos un frame")
    }
    pub fn first(&self) -> &Frame {
        &self.frames[0]
    }
    pub fn name(&self, h: i32) -> &str {
        &self.world.a(h).name
    }
    pub fn door_attempt(&self) -> Option<(f64, bool)> {
        self.events.iter().find_map(|e| match e {
            Event::DoorAttempt { t, allowed, .. } => Some((*t, *allowed)),
            _ => None,
        })
    }
    pub fn choice(&self) -> Option<i32> {
        self.events.iter().find_map(|e| match e {
            Event::Choice { target, .. } => Some(*target),
            _ => None,
        })
    }
    pub fn min_dist2d(&self, a: i32, b: i32) -> f64 {
        self.frames.iter().map(|f| f.actors[a as usize].loc.dist2d(f.actors[b as usize].loc)).fold(f64::INFINITY, f64::min)
    }
    pub fn max_displacement(&self, h: i32) -> f64 {
        let start = self.first().actors[h as usize].loc;
        self.frames.iter().map(|f| f.actors[h as usize].loc.dist2d(start)).fold(0.0, f64::max)
    }
    /// Primer instante en que se cumple la condición.
    pub fn first_time(&self, mut pred: impl FnMut(&Frame) -> bool) -> Option<f64> {
        self.frames.iter().find(|f| pred(f)).map(|f| f.t)
    }
}

#[derive(Serialize)]
pub struct ActorMeta {
    pub handle: i32,
    pub class: Class,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub walls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sight: Option<[f64; 2]>,
}

#[derive(Serialize)]
pub struct Trace {
    pub dt: f64,
    pub duration: f64,
    pub actors: Vec<ActorMeta>,
    /// Por frame: [x, y, z, yaw, leafYaw, hidden, flag] por actor.
    pub frames: Vec<Vec<f64>>,
    pub events: Vec<Event>,
    pub draws: Vec<Draw>,
}

fn snapshot(world: &World) -> Frame {
    Frame {
        t: world.time,
        actors: world
            .actors
            .iter()
            .map(|a| ActorFrame { loc: a.loc, yaw: a.rot.yaw, leaf: a.comp_rot[0].yaw, hidden: a.hidden, flag: a.flag, speed: a.velocity.len2d() })
            .collect(),
    }
}

pub fn run(compiled: &Compiled, scenario: &mut Scenario) -> Record {
    let world = std::mem::take(&mut scenario.world);
    let dt = world.dt;
    let mut sim = match compiled.instantiate(world) {
        Ok(sim) => sim,
        Err(e) => {
            let w = World::new();
            return Record { frames: vec![snapshot(&w)], events: vec![], crash: Some(e), world: w };
        }
    };
    let handles: Vec<(i32, Class)> = sim.world().actors.iter().map(|a| (a.handle, a.class)).collect();
    let mut frames = Vec::new();

    let setup = (|| -> Result<(), String> {
        for &(h, class) in &handles {
            if sim.create(class as i32, h)? == 0 {
                return Err(format!("No se pudo crear el actor {h}"));
            }
        }
        for &(slot, a, b) in &scenario.wires {
            sim.wire(slot, a, b)?;
        }
        for &(h, _) in &handles {
            sim.begin_play(h)?;
        }
        Ok(())
    })();
    frames.push(snapshot(sim.world()));
    let mut crash = setup.err();

    let steps = (scenario.duration / dt).round() as usize;
    if crash.is_none() {
        'frames: for _ in 0..steps {
            sim.world_mut().time += dt;
            {
                let mut ctx = Ctx { sim: &mut sim };
                if let Err(e) = (scenario.script)(&mut ctx) {
                    crash = Some(e);
                    break;
                }
            }
            for &(h, _) in &handles {
                if sim.world().a(h).destroyed {
                    continue;
                }
                if let Err(e) = sim.tick(h, dt as f32) {
                    crash = Some(e);
                    break 'frames;
                }
            }
            if let Err(e) = sim.tick_timers(dt as f32) {
                crash = Some(e);
                break;
            }
            sim.world_mut().integrate();

            // Recoger llaves al pasar por encima.
            let mut pickups = Vec::new();
            {
                let w = sim.world();
                for key in w.actors.iter().filter(|a| a.class == Class::Key && !a.hidden && !a.destroyed) {
                    if let Some(who) = w.actors.iter().find(|a| a.class == Class::Monigote && a.loc.dist2d(key.loc) < 55.0) {
                        pickups.push((who.handle, key.handle));
                    }
                }
            }
            for (who, key) in pickups {
                let w = sim.world_mut();
                w.am(key).hidden = true;
                let t = w.time;
                w.push_event(Event::Pickup { t, who, key });
                w.bubble(who, "¡Llave!", "#facc15");
                if let Err(e) = sim.call(FN_ON_KEY_PICKED, who, key) {
                    crash = Some(e);
                    break 'frames;
                }
            }

            frames.push(snapshot(sim.world()));
            let w = sim.world();
            if let Some(f) = w.finished_at {
                if w.time > f + 1.2 {
                    break;
                }
            }
        }
    }

    let mut world = sim.into_world();
    if let Some(msg) = &crash {
        let t = world.time;
        world.events.push(Event::Crash { t, message: msg.clone() });
    }
    let events = world.events.clone();
    Record { frames, events, crash, world }
}

pub fn trace(record: &Record) -> Trace {
    let actors = record
        .world
        .actors
        .iter()
        .map(|a| ActorMeta {
            handle: a.handle,
            class: a.class,
            name: a.name.clone(),
            label: a.label.clone(),
            walls: a.walls && a.class == Class::Door,
            sight: a.sight.map(|(r, h)| [r, h]),
        })
        .collect();
    let frames = record
        .frames
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0 || *i == record.frames.len() - 1)
        .map(|(_, f)| {
            let mut v = Vec::with_capacity(1 + f.actors.len() * 7);
            v.push(r2(f.t));
            for a in &f.actors {
                v.extend_from_slice(&[r2(a.loc.x), r2(a.loc.y), r2(a.loc.z), r2(a.yaw), r2(a.leaf), a.hidden as u8 as f64, a.flag as f64]);
            }
            v
        })
        .collect();
    Trace {
        dt: record.world.dt * 2.0,
        duration: record.last().t,
        actors,
        frames,
        events: record.events.clone(),
        draws: record.world.draws.clone(),
    }
}
