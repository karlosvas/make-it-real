//! Ejecuta el wasm del usuario con wasmi e implementa los imports `ue.*` del mini-Unreal.

use super::world::{Draw, Event, Rot, V3, World, color_hex};
use wasmi::{Caller, Config, Engine, Error, Extern, Linker, Memory, Module, Store, StoreLimits, StoreLimitsBuilder, TrapCode, TypedFunc};

pub struct HostState {
    pub world: World,
    limits: StoreLimits,
}

/// Módulo compilado una vez y reutilizado para instanciar cada escenario con memoria limpia.
pub struct Compiled {
    engine: Engine,
    module: Module,
    linker: Linker<HostState>,
}

pub struct Sim {
    pub store: Store<HostState>,
    f_create: TypedFunc<(i32, i32), i32>,
    f_wire: TypedFunc<(i32, i32, i32), ()>,
    f_call: TypedFunc<(i32, i32, i32), i32>,
    f_begin: TypedFunc<i32, ()>,
    f_tick: TypedFunc<(i32, f32), ()>,
    f_timers: TypedFunc<f32, ()>,
}

/// Combustible por escenario: si el código del usuario se pasa, lo cortamos (bucle infinito).
pub const FUEL_PER_SCENARIO: u64 = 60_000_000;

fn actor_err() -> Error {
    Error::new("Access violation: actor inválido (¿un puntero roto?)")
}

fn memory(caller: &Caller<'_, HostState>) -> Result<Memory, Error> {
    caller.get_export("memory").and_then(Extern::into_memory).ok_or_else(|| Error::new("el módulo no exporta memoria"))
}

fn write_f64s(caller: &mut Caller<'_, HostState>, ptr: i32, values: &[f64]) -> Result<(), Error> {
    let mem = memory(caller)?;
    let mut bytes = Vec::with_capacity(values.len() * 8);
    for v in values {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    mem.write(&mut *caller, ptr as u32 as usize, &bytes).map_err(|_| Error::new("Access violation: escritura fuera de la memoria"))
}

fn read_str(caller: &Caller<'_, HostState>, ptr: i32, len: i32) -> Result<String, Error> {
    let mem = memory(caller)?;
    let data = mem.data(caller);
    let start = ptr as u32 as usize;
    let end = start + (len.max(0) as usize).min(4096);
    data.get(start..end)
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .ok_or_else(|| Error::new("Access violation: lectura fuera de la memoria"))
}

fn define_imports(linker: &mut Linker<HostState>) -> Result<(), Error> {
    linker.func_wrap("ue", "sin", |x: f64| x.sin())?;
    linker.func_wrap("ue", "cos", |x: f64| x.cos())?;
    linker.func_wrap("ue", "tan", |x: f64| x.tan())?;
    linker.func_wrap("ue", "asin", |x: f64| x.asin())?;
    linker.func_wrap("ue", "acos", |x: f64| x.acos())?;
    linker.func_wrap("ue", "atan", |x: f64| x.atan())?;
    linker.func_wrap("ue", "atan2", |y: f64, x: f64| y.atan2(x))?;
    linker.func_wrap("ue", "pow", |a: f64, b: f64| a.powf(b))?;
    linker.func_wrap("ue", "exp", |x: f64| x.exp())?;
    linker.func_wrap("ue", "ln", |x: f64| x.ln())?;
    linker.func_wrap("ue", "fmod", |a: f64, b: f64| a % b)?;

    linker.func_wrap("ue", "get_location", |mut c: Caller<'_, HostState>, h: i32, out: i32| -> Result<(), Error> {
        let l = c.data().world.actor(h).ok_or_else(actor_err)?.loc;
        write_f64s(&mut c, out, &[l.x, l.y, l.z])
    })?;
    linker.func_wrap("ue", "set_location", |mut c: Caller<'_, HostState>, h: i32, x: f64, y: f64, z: f64, _sweep: i32| -> Result<(), Error> {
        let a = c.data_mut().world.actor_mut(h).ok_or_else(actor_err)?;
        if x.is_finite() && y.is_finite() && z.is_finite() {
            a.loc = V3::new(x, y, z);
        }
        Ok(())
    })?;
    linker.func_wrap("ue", "get_rotation", |mut c: Caller<'_, HostState>, h: i32, out: i32| -> Result<(), Error> {
        let r = c.data().world.actor(h).ok_or_else(actor_err)?.rot;
        write_f64s(&mut c, out, &[r.pitch, r.yaw, r.roll])
    })?;
    linker.func_wrap("ue", "set_rotation", |mut c: Caller<'_, HostState>, h: i32, p: f64, y: f64, r: f64| -> Result<(), Error> {
        let a = c.data_mut().world.actor_mut(h).ok_or_else(actor_err)?;
        if p.is_finite() && y.is_finite() && r.is_finite() {
            a.rot = Rot { pitch: p, yaw: y, roll: r };
        }
        Ok(())
    })?;
    linker.func_wrap("ue", "get_velocity", |mut c: Caller<'_, HostState>, h: i32, out: i32| -> Result<(), Error> {
        let v = c.data().world.actor(h).ok_or_else(actor_err)?.velocity;
        write_f64s(&mut c, out, &[v.x, v.y, v.z])
    })?;
    linker.func_wrap(
        "ue",
        "add_movement_input",
        |mut c: Caller<'_, HostState>, h: i32, x: f64, y: f64, z: f64, scale: f64, max_speed: f64, orient: i32| -> Result<(), Error> {
            let a = c.data_mut().world.actor_mut(h).ok_or_else(actor_err)?;
            let v = V3::new(x, y, z) * scale;
            if v.x.is_finite() && v.y.is_finite() && v.z.is_finite() {
                a.input += v;
            }
            a.max_speed = if max_speed.is_finite() { max_speed.clamp(0.0, 3000.0) } else { 400.0 };
            a.orient = orient != 0;
            Ok(())
        },
    )?;
    linker.func_wrap("ue", "get_comp_rotation", |mut c: Caller<'_, HostState>, h: i32, idx: i32, out: i32| -> Result<(), Error> {
        let a = c.data().world.actor(h).ok_or_else(actor_err)?;
        let r = a.comp_rot.get(idx as usize).copied().unwrap_or_default();
        write_f64s(&mut c, out, &[r.pitch, r.yaw, r.roll])
    })?;
    linker.func_wrap("ue", "set_comp_rotation", |mut c: Caller<'_, HostState>, h: i32, idx: i32, p: f64, y: f64, r: f64| -> Result<(), Error> {
        let a = c.data_mut().world.actor_mut(h).ok_or_else(actor_err)?;
        if let Some(slot) = a.comp_rot.get_mut(idx as usize) {
            if p.is_finite() && y.is_finite() && r.is_finite() {
                *slot = Rot { pitch: p, yaw: y, roll: r };
            }
        }
        a.door_anim = None;
        Ok(())
    })?;
    linker.func_wrap("ue", "destroy", |mut c: Caller<'_, HostState>, h: i32| -> Result<i32, Error> {
        let a = c.data_mut().world.actor_mut(h).ok_or_else(actor_err)?;
        a.destroyed = true;
        a.hidden = true;
        Ok(1)
    })?;
    linker.func_wrap("ue", "set_hidden", |mut c: Caller<'_, HostState>, h: i32, hidden: i32| -> Result<(), Error> {
        c.data_mut().world.actor_mut(h).ok_or_else(actor_err)?.hidden = hidden != 0;
        Ok(())
    })?;
    linker.func_wrap("ue", "is_hidden", |c: Caller<'_, HostState>, h: i32| -> Result<i32, Error> {
        Ok(c.data().world.actor(h).ok_or_else(actor_err)?.hidden as i32)
    })?;
    linker.func_wrap("ue", "get_name", |mut c: Caller<'_, HostState>, h: i32, buf: i32, cap: i32| -> Result<i32, Error> {
        let name = c.data().world.actor(h).ok_or_else(actor_err)?.name.clone();
        let bytes = &name.as_bytes()[..name.len().min(cap.max(0) as usize)];
        let mem = memory(&c)?;
        mem.write(&mut c, buf as u32 as usize, bytes).map_err(|_| Error::new("Access violation"))?;
        Ok(bytes.len() as i32)
    })?;
    linker.func_wrap("ue", "time", |c: Caller<'_, HostState>| c.data().world.time)?;
    linker.func_wrap("ue", "delta", |c: Caller<'_, HostState>| c.data().world.dt)?;
    linker.func_wrap("ue", "log", |mut c: Caller<'_, HostState>, cat: i32, cat_len: i32, verbosity: i32, msg: i32, len: i32| -> Result<(), Error> {
        let category = read_str(&c, cat, cat_len)?;
        let text = read_str(&c, msg, len)?;
        c.data_mut().world.log(&category, verbosity, text);
        Ok(())
    })?;
    linker.func_wrap("ue", "screen", |mut c: Caller<'_, HostState>, color: i32, duration: f64, msg: i32, len: i32| -> Result<(), Error> {
        let text = read_str(&c, msg, len)?;
        let w = &mut c.data_mut().world;
        if w.log_count < super::world::MAX_LOGS {
            w.log_count += 1;
            let t = w.time;
            w.push_event(Event::Screen { t, text, color: color_hex(color as u32), duration: if duration > 0.0 { duration.min(30.0) } else { 2.0 } });
        }
        Ok(())
    })?;
    linker.func_wrap("ue", "fatal", |mut c: Caller<'_, HostState>, msg: i32, len: i32| -> Result<(), Error> {
        let text = read_str(&c, msg, len)?;
        c.data_mut().world.fatal_message = Some(text.clone());
        Err(Error::new(text))
    })?;
    linker.func_wrap(
        "ue",
        "draw",
        |mut c: Caller<'_, HostState>, kind: i32, ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64, size: f64, extra: f64, color: i32, life: f64, persistent: i32| {
            let w = &mut c.data_mut().world;
            let t = w.time;
            w.draw(Draw {
                t,
                life,
                persistent: persistent != 0,
                kind: kind.clamp(0, 9) as u8,
                a: V3::new(ax, ay, az).arr(),
                b: V3::new(bx, by, bz).arr(),
                size: if size.is_finite() { size } else { 0.0 },
                extra: if extra.is_finite() { extra } else { 0.0 },
                color: color_hex(color as u32),
                text: None,
            });
        },
    )?;
    linker.func_wrap(
        "ue",
        "draw_text",
        |mut c: Caller<'_, HostState>, x: f64, y: f64, z: f64, ptr: i32, len: i32, color: i32, life: f64| -> Result<(), Error> {
            let text = read_str(&c, ptr, len)?;
            let w = &mut c.data_mut().world;
            let t = w.time;
            w.draw(Draw { t, life, persistent: false, kind: 9, a: V3::new(x, y, z).arr(), b: [0.0; 3], size: 0.0, extra: 0.0, color: color_hex(color as u32), text: Some(text) });
            Ok(())
        },
    )?;
    Ok(())
}

impl Compiled {
    pub fn new(wasm: &[u8]) -> Result<Self, String> {
        let mut config = Config::default();
        config.consume_fuel(true);
        let engine = Engine::new(&config);
        let module = Module::new(&engine, wasm).map_err(|e| format!("wasm inválido: {e}"))?;
        let mut linker = Linker::new(&engine);
        define_imports(&mut linker).map_err(|e| format!("imports: {e}"))?;
        Ok(Self { engine, module, linker })
    }

    pub fn instantiate(&self, world: World) -> Result<Sim, String> {
        let limits = StoreLimitsBuilder::new().memory_size(64 * 1024 * 1024).instances(1).build();
        let mut store = Store::new(&self.engine, HostState { world, limits });
        store.limiter(|s| &mut s.limits);
        store.set_fuel(FUEL_PER_SCENARIO).map_err(|e| e.to_string())?;
        let instance = self.linker.instantiate_and_start(&mut store, &self.module).map_err(|e| format!("No se pudo cargar tu código: {e}"))?;
        let get = |name: &str| format!("falta el export {name}");
        if let Ok(ctors) = instance.get_typed_func::<(), ()>(&store, "__wasm_call_ctors") {
            ctors.call(&mut store, ()).map_err(|e| format!("Crash en constructores globales: {e}"))?;
        }
        Ok(Sim {
            f_create: instance.get_typed_func(&store, "mir_create").map_err(|_| get("mir_create"))?,
            f_wire: instance.get_typed_func(&store, "mir_wire").map_err(|_| get("mir_wire"))?,
            f_call: instance.get_typed_func(&store, "mir_call").map_err(|_| get("mir_call"))?,
            f_begin: instance.get_typed_func(&store, "mir_begin_play").map_err(|_| get("mir_begin_play"))?,
            f_tick: instance.get_typed_func(&store, "mir_tick").map_err(|_| get("mir_tick"))?,
            f_timers: instance.get_typed_func(&store, "mir_tick_timers").map_err(|_| get("mir_tick_timers"))?,
            store,
        })
    }
}

impl Sim {
    pub fn world(&self) -> &World {
        &self.store.data().world
    }
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.store.data_mut().world
    }
    pub fn into_world(self) -> World {
        self.store.into_data().world
    }

    fn friendly(&mut self, e: Error) -> String {
        if let Some(msg) = self.world_mut().fatal_message.take() {
            return msg;
        }
        let fuel_left = self.store.get_fuel().unwrap_or(1);
        match e.as_trap_code() {
            Some(TrapCode::OutOfFuel) => "Tu código tarda demasiado y se ha cortado la ejecución (¿un bucle infinito?)".into(),
            _ if fuel_left == 0 => "Tu código tarda demasiado y se ha cortado la ejecución (¿un bucle infinito?)".into(),
            Some(TrapCode::StackOverflow) => "Stack overflow: demasiadas llamadas anidadas (¿una recursión infinita?)".into(),
            Some(TrapCode::IndirectCallToNull | TrapCode::BadSignature | TrapCode::TableOutOfBounds) => {
                "Access violation: llamada a través de un puntero nulo o inválido (¿usas un puntero sin comprobar si es nullptr?)".into()
            }
            Some(TrapCode::MemoryOutOfBounds) => "Access violation: acceso a memoria inválida (¿un puntero roto?)".into(),
            Some(TrapCode::IntegerDivisionByZero) => "Integer divide by zero: división entera entre cero".into(),
            Some(TrapCode::IntegerOverflow) => "Integer overflow".into(),
            Some(TrapCode::UnreachableCodeReached) => "Crash: se ha ejecutado código inalcanzable".into(),
            _ => format!("Crash: {e}"),
        }
    }

    pub fn create(&mut self, class: i32, handle: i32) -> Result<i32, String> {
        let f = self.f_create;
        f.call(&mut self.store, (class, handle)).map_err(|e| self.friendly(e))
    }
    pub fn wire(&mut self, slot: i32, a: i32, b: i32) -> Result<(), String> {
        let f = self.f_wire;
        f.call(&mut self.store, (slot, a, b)).map_err(|e| self.friendly(e))
    }
    pub fn call(&mut self, function: i32, a: i32, b: i32) -> Result<i32, String> {
        let f = self.f_call;
        f.call(&mut self.store, (function, a, b)).map_err(|e| self.friendly(e))
    }
    pub fn begin_play(&mut self, h: i32) -> Result<(), String> {
        let f = self.f_begin;
        f.call(&mut self.store, h).map_err(|e| self.friendly(e))
    }
    pub fn tick(&mut self, h: i32, dt: f32) -> Result<(), String> {
        let f = self.f_tick;
        f.call(&mut self.store, (h, dt)).map_err(|e| self.friendly(e))
    }
    pub fn tick_timers(&mut self, dt: f32) -> Result<(), String> {
        let f = self.f_timers;
        f.call(&mut self.store, dt).map_err(|e| self.friendly(e))
    }
}
