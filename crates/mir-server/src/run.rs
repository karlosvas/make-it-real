//! Compilar + ejecutar todos los escenarios de un ejercicio.

use crate::compile::{self, Diagnostic};
use crate::exercise;
use crate::exercises;
use crate::sim::host::Compiled;
use crate::sim::runner::{self, Trace};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

#[derive(Serialize)]
pub struct ScenarioResult {
    pub id: String,
    pub name: String,
    pub passed: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crash: Option<String>,
    pub trace: Trace,
}

#[derive(Serialize)]
#[serde(tag = "stage", rename_all = "snake_case")]
pub enum RunOutcome {
    CompileError { diagnostics: Vec<Diagnostic>, log: String, compile_ms: u64 },
    Ran { diagnostics: Vec<Diagnostic>, log: String, compile_ms: u64, run_ms: u64, all_passed: bool, scenarios: Vec<ScenarioResult> },
}

pub fn run_exercise(root: &Path, id: &str, files: &HashMap<String, String>) -> Result<RunOutcome, String> {
    let (meta, dir) = exercise::load(root, id)?;
    let out = compile::compile(root, &dir, &meta, files)?;
    let Some(wasm) = out.wasm else {
        return Ok(RunOutcome::CompileError { diagnostics: out.diagnostics, log: out.log, compile_ms: out.ms });
    };
    let start = Instant::now();
    let compiled = Compiled::new(&wasm)?;
    let registry = exercises::scenarios(&meta.id).ok_or_else(|| format!("{id} no tiene escenarios"))?;
    let mut results = Vec::new();
    for s in &meta.scenarios {
        let builder = registry.iter().find(|(sid, _)| *sid == s.id).map(|(_, b)| *b).ok_or_else(|| format!("falta el escenario {}", s.id))?;
        let mut scenario = builder();
        let record = runner::run(&compiled, &mut scenario);
        let (passed, message) = match &record.crash {
            Some(c) => (false, c.clone()),
            None => match (scenario.check)(&record) {
                Ok(m) => (true, m),
                Err(m) => (false, m),
            },
        };
        results.push(ScenarioResult { id: s.id.clone(), name: s.name.clone(), passed, message, crash: record.crash.clone(), trace: runner::trace(&record) });
    }
    let all_passed = results.iter().all(|r| r.passed);
    Ok(RunOutcome::Ran { diagnostics: out.diagnostics, log: out.log, compile_ms: out.ms, run_ms: start.elapsed().as_millis() as u64, all_passed, scenarios: results })
}
