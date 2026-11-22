//! Compila el C++ del usuario a WebAssembly con clang contra el mini-Unreal.

use crate::exercise::ExerciseMeta;
use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, Serialize)]
pub struct Diagnostic {
    pub file: String,
    pub line: u32,
    pub col: u32,
    pub severity: String,
    pub message: String,
}

pub struct CompileOutput {
    pub wasm: Option<Vec<u8>>,
    pub diagnostics: Vec<Diagnostic>,
    pub log: String,
    pub ms: u64,
}

const MAX_FILE_BYTES: usize = 40 * 1024;
const COMPILE_TIMEOUT: Duration = Duration::from_secs(15);

fn collect_headers(dir: &Path, prefix: &str, out: &mut BTreeSet<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let path = e.path();
            if path.is_dir() {
                if name != "Mir" {
                    collect_headers(&path, &format!("{prefix}{name}/"), out);
                }
            } else if name.ends_with(".h") {
                out.insert(format!("{prefix}{name}"));
            }
        }
    }
}

fn directive_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r#"^\s*(?:#|%:)\s*([A-Za-z_]+)\s*(.*)$"#).unwrap())
}

fn diag_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^(?P<file>[^:\n]+):(?P<line>\d+):(?P<col>\d+): (?P<sev>fatal error|error|warning|note): (?P<msg>.*)$").unwrap())
}

/// Revisa los `#include` y otras directivas peligrosas antes de llamar a clang.
fn validate(file: &str, source: &str, allowed: &BTreeSet<String>) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    let joined = source.replace("\\\r\n", "").replace("\\\n", "");
    // Las continuaciones de línea cambian la numeración; aproximamos con la línea original.
    for (i, line) in joined.lines().enumerate() {
        let Some(c) = directive_re().captures(line) else { continue };
        let directive = c.get(1).unwrap().as_str();
        let rest = c.get(2).unwrap().as_str().trim();
        let err = |msg: String| Diagnostic { file: file.into(), line: i as u32 + 1, col: 1, severity: "error".into(), message: msg };
        match directive {
            "include" => {
                let target = rest.trim_start_matches(['"', '<']).split(['"', '>']).next().unwrap_or("").trim();
                if target.contains("..") || target.starts_with('/') || target.contains('\\') || target.contains(':') {
                    out.push(err(format!("No se permite incluir '{target}'.")));
                } else if !allowed.contains(target) {
                    let mut msg = format!("No existe la cabecera '{target}' en este mini-Unreal.");
                    if target.ends_with(".generated.h") {
                        msg = format!("'{target}' lo genera UnrealHeaderTool en Unreal; aquí no hace falta, quita esa línea.");
                    }
                    out.push(err(msg));
                }
            }
            "include_next" | "import" | "embed" => out.push(err(format!("La directiva #{directive} no está permitida."))),
            _ => {}
        }
    }
    if source.contains("__has_embed") {
        out.push(Diagnostic { file: file.into(), line: 1, col: 1, severity: "error".into(), message: "__has_embed no está permitido.".into() });
    }
    out
}

pub fn compile(root: &Path, ex_dir: &Path, meta: &ExerciseMeta, files: &HashMap<String, String>) -> Result<CompileOutput, String> {
    let start = Instant::now();
    let editable: Vec<&str> = meta.files.iter().filter(|f| f.editable).map(|f| f.name.as_str()).collect();
    for name in files.keys() {
        if !editable.contains(&name.as_str()) {
            return Err(format!("El archivo {name} no se puede editar en este ejercicio"));
        }
    }

    let sdk_include = root.join("sdk/include");
    let mut allowed = BTreeSet::new();
    collect_headers(&sdk_include, "", &mut allowed);
    for f in &meta.files {
        if f.name.ends_with(".h") {
            allowed.insert(f.name.clone());
        }
    }

    let tmp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let mut diagnostics = Vec::new();
    let mut sources = Vec::new();
    for f in meta.files.iter().filter(|f| f.editable) {
        let content = match files.get(&f.name) {
            Some(c) => c.clone(),
            None => std::fs::read_to_string(ex_dir.join(&f.name)).map_err(|e| e.to_string())?,
        };
        if content.len() > MAX_FILE_BYTES {
            return Err(format!("{} es demasiado grande (máximo 40 KB)", f.name));
        }
        diagnostics.extend(validate(&f.name, &content, &allowed));
        std::fs::write(tmp.path().join(&f.name), content).map_err(|e| e.to_string())?;
        if f.name.ends_with(".cpp") {
            sources.push(f.name.clone());
        }
    }
    if !diagnostics.is_empty() {
        return Ok(CompileOutput { wasm: None, log: String::new(), diagnostics, ms: start.elapsed().as_millis() as u64 });
    }

    let out_path = tmp.path().join("out.wasm");
    let mut cmd = Command::new(std::env::var("MIR_CLANG").unwrap_or_else(|_| "clang++".into()));
    cmd.current_dir(tmp.path())
        .args([
            "--target=wasm32",
            "-std=c++20",
            "-O1",
            "-mbulk-memory",
            "-nostdinc",
            "-nostdlib",
            "-fno-exceptions",
            "-fno-rtti",
            "-fno-threadsafe-statics",
            "-fno-delete-null-pointer-checks",
            "-fno-color-diagnostics",
            "-ferror-limit=25",
            "-Wall",
            "-Wno-unused-parameter",
            "-Wno-unused-variable",
            "-Wno-unused-private-field",
        ])
        .arg("-I")
        .arg(&sdk_include)
        .arg("-I")
        .arg(ex_dir)
        .args(&sources)
        .arg(ex_dir.join("glue.cpp"));
    for entry in std::fs::read_dir(root.join("sdk/runtime")).map_err(|e| e.to_string())?.flatten() {
        if entry.path().extension().is_some_and(|e| e == "cpp") {
            cmd.arg(entry.path());
        }
    }
    cmd.args(["-Wl,--no-entry", "-Wl,--export=__wasm_call_ctors", "-Wl,-z,stack-size=262144", "-o"]).arg(&out_path);
    cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| format!("No se pudo ejecutar clang++: {e}"))?;
    let deadline = Instant::now() + COMPILE_TIMEOUT;
    let status = loop {
        if let Some(status) = child.try_wait().map_err(|e| e.to_string())? {
            break status;
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            return Err("La compilación ha tardado demasiado".into());
        }
        std::thread::sleep(Duration::from_millis(5));
    };
    let output = child.wait_with_output().map_err(|e| e.to_string())?;
    let mut log = String::from_utf8_lossy(&output.stderr).to_string();

    // Rutas más cortas y legibles.
    let replacements = [
        (format!("{}/", ex_dir.display()), String::new()),
        (format!("{}/", sdk_include.display()), String::new()),
        (format!("{}/", root.join("sdk/runtime").display()), "runtime/".to_string()),
        (format!("{}/", tmp.path().display()), String::new()),
    ];
    for (from, to) in &replacements {
        log = log.replace(from.as_str(), to);
    }

    let mut diagnostics = Vec::new();
    for line in log.lines() {
        if let Some(c) = diag_re().captures(line) {
            let sev = c["sev"].replace("fatal error", "error");
            if sev == "note" {
                continue;
            }
            diagnostics.push(Diagnostic {
                file: c["file"].to_string(),
                line: c["line"].parse().unwrap_or(1),
                col: c["col"].parse().unwrap_or(1),
                severity: sev,
                message: c["msg"].to_string(),
            });
        } else if let Some(sym) = line.split("undefined symbol: ").nth(1) {
            diagnostics.push(Diagnostic {
                file: String::new(),
                line: 0,
                col: 0,
                severity: "error".into(),
                message: format!("Falta la definición de `{sym}`. ¿Has borrado o renombrado una función que el ejercicio necesita?"),
            });
        }
    }
    let ms = start.elapsed().as_millis() as u64;
    if !status.success() {
        if !diagnostics.iter().any(|d| d.severity == "error") {
            diagnostics.push(Diagnostic { file: String::new(), line: 0, col: 0, severity: "error".into(), message: "La compilación ha fallado (mira la salida del compilador).".into() });
        }
        return Ok(CompileOutput { wasm: None, diagnostics, log, ms });
    }
    let wasm = std::fs::read(&out_path).map_err(|e| e.to_string())?;
    Ok(CompileOutput { wasm: Some(wasm), diagnostics, log, ms })
}
