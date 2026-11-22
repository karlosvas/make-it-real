//! 03 · Solo desde delante — ADoor::CanOpen con el producto escalar.

use super::Builder;
use super::common::*;
use crate::sim::runner::Scenario;
use crate::sim::world::{Class, V3, World};

pub fn scenarios() -> Vec<(&'static str, Builder)> {
    vec![("delante", delante), ("detras", detras), ("diagonal-delante", diagonal_delante), ("diagonal-detras", diagonal_detras)]
}

fn build(start: V3, approach: V3, expected: bool, situation: &'static str) -> Scenario {
    let mut w = World::new();
    let door = w.spawn(Class::Door, "Puerta", V3::ZERO, 0.0);
    let moni = w.spawn(Class::Monigote, "Monigote", start, yaw_towards(start, approach));
    let through = if approach.x > 0.0 { V3::xy(-260.0, 0.0) } else { V3::xy(260.0, 0.0) };
    Scenario {
        duration: 10.0,
        world: w,
        wires: vec![],
        script: door_attempt_script(DoorAttempt { moni, door, path: vec![], approach, facing: Facing::Door, through }),
        check: expect_door(expected, situation),
    }
}

fn delante() -> Scenario {
    build(V3::xy(520.0, 160.0), V3::xy(120.0, 0.0), true, "Desde delante")
}
fn detras() -> Scenario {
    build(V3::xy(-520.0, -160.0), V3::xy(-120.0, 0.0), false, "Desde detrás")
}
fn diagonal_delante() -> Scenario {
    build(V3::xy(420.0, -420.0), V3::xy(110.0, -95.0), true, "En diagonal por delante")
}
fn diagonal_detras() -> Scenario {
    build(V3::xy(-420.0, 420.0), V3::xy(-110.0, 95.0), false, "En diagonal por detrás")
}
