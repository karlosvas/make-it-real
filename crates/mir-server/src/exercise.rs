//! Metadatos de los ejercicios (exercises/<id>/exercise.json).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileMeta {
    pub name: String,
    pub editable: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScenarioMeta {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExerciseMeta {
    pub id: String,
    pub number: u32,
    pub title: String,
    pub difficulty: String,
    pub summary: String,
    pub files: Vec<FileMeta>,
    pub scenarios: Vec<ScenarioMeta>,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default)]
    pub api: Vec<String>,
}

/// Raíz del repo (donde están `sdk/` y `exercises/`).
pub fn root() -> PathBuf {
    if let Ok(r) = std::env::var("MIR_ROOT") {
        return PathBuf::from(r);
    }
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn valid_id(id: &str) -> bool {
    !id.is_empty() && id.len() < 80 && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

pub fn load(root: &Path, id: &str) -> Result<(ExerciseMeta, PathBuf), String> {
    if !valid_id(id) {
        return Err(format!("Ejercicio inválido: {id}"));
    }
    let dir = root.join("exercises").join(id);
    let raw = std::fs::read_to_string(dir.join("exercise.json")).map_err(|_| format!("No existe el ejercicio {id}"))?;
    let meta: ExerciseMeta = serde_json::from_str(&raw).map_err(|e| format!("exercise.json de {id} inválido: {e}"))?;
    Ok((meta, dir))
}

pub fn list(root: &Path) -> Vec<ExerciseMeta> {
    let mut out = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root.join("exercises")) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if let Ok((meta, _)) = load(root, &name) {
                out.push(meta);
            }
        }
    }
    out.sort_by_key(|m| m.number);
    out
}
