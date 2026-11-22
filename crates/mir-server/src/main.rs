//! mir-server: compila el C++ de los ejercicios a wasm, lo ejecuta en el mundo
//! simulado y devuelve resultados + traza. También gestiona el login con GitHub,
//! guarda el progreso y sirve la web ya construida.

mod auth;
mod compile;
mod db;
mod exercise;
mod exercises;
mod run;
mod sim;
mod store;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, post, put};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub root: Arc<PathBuf>,
    pub gate: Arc<tokio::sync::Semaphore>,
    pub store: Arc<store::Store>,
    pub github: Option<auth::GithubConfig>,
    pub public_url: Option<String>,
    pub http: reqwest::Client,
}

#[derive(Deserialize)]
struct RunRequest {
    exercise: String,
    #[serde(default)]
    files: HashMap<String, String>,
}

fn error(status: StatusCode, msg: impl Into<String>) -> Response {
    (status, Json(serde_json::json!({ "error": msg.into() }))).into_response()
}

async fn health() -> &'static str {
    "ok"
}

async fn list_exercises(State(state): State<AppState>) -> Response {
    Json(exercise::list(&state.root)).into_response()
}

async fn run_handler(State(state): State<AppState>, headers: HeaderMap, Json(req): Json<RunRequest>) -> Response {
    let Ok(_permit) = state.gate.clone().acquire_owned().await else {
        return error(StatusCode::SERVICE_UNAVAILABLE, "servidor ocupado");
    };
    let user = auth::current_user(&state, &headers).await;
    let root = state.root.clone();
    let exercise_id = req.exercise.clone();
    let files = req.files.clone();
    let result = tokio::task::spawn_blocking(move || run::run_exercise(&root, &req.exercise, &req.files)).await;
    match result {
        Ok(Ok(outcome)) => {
            // Con sesión iniciada, el servidor apunta el código y si está resuelto.
            if let Some(user) = user {
                if !files.is_empty() {
                    let _ = state.store.save_files(user.id, &exercise_id, &files).await;
                }
                if matches!(outcome, run::RunOutcome::Ran { all_passed: true, .. }) {
                    let _ = state.store.mark_solved(user.id, &exercise_id).await;
                }
            }
            Json(outcome).into_response()
        }
        Ok(Err(msg)) => error(StatusCode::BAD_REQUEST, msg),
        Err(e) => error(StatusCode::INTERNAL_SERVER_ERROR, format!("fallo interno: {e}")),
    }
}

/// Lee `.env` y `.env.local` de la raíz (KEY=VALUE). Las variables de entorno reales tienen prioridad.
fn load_env(root: &Path) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for text in [".env.local", ".env"].iter().filter_map(|f| std::fs::read_to_string(root.join(f)).ok()) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                map.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
            }
        }
    }
    map
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = exercise::root();
    let env = load_env(&root);
    let var = |k: &str| std::env::var(k).ok().or_else(|| env.get(k).cloned()).filter(|v| !v.is_empty());

    let port: u16 = var("PORT").or_else(|| var("MIR_PORT")).and_then(|p| p.parse().ok()).unwrap_or(8787);
    let host: std::net::IpAddr = var("MIR_HOST").and_then(|h| h.parse().ok()).unwrap_or([127, 0, 0, 1].into());
    let public_url = var("MIR_PUBLIC_URL");
    let github = match (var("MIR_GITHUB_CLIENT_ID"), var("MIR_GITHUB_CLIENT_SECRET")) {
        (Some(client_id), Some(client_secret)) => Some(auth::GithubConfig { client_id, client_secret }),
        _ => None,
    };
    let http = reqwest::Client::builder().timeout(std::time::Duration::from_secs(15)).build()?;
    let redis_url = var("KV_REST_API_URL").or_else(|| var("UPSTASH_REDIS_REST_URL"));
    let redis_token = var("KV_REST_API_TOKEN").or_else(|| var("UPSTASH_REDIS_REST_TOKEN"));
    let store = match (redis_url, redis_token) {
        (Some(url), Some(token)) => store::Store::Redis(store::Redis::new(url, token, http.clone())),
        _ => store::Store::Sqlite(db::Db::open(&root.join("data/mir.db"))?),
    };

    let state = AppState {
        root: Arc::new(root.clone()),
        gate: Arc::new(tokio::sync::Semaphore::new(4)),
        store: Arc::new(store),
        github: github.clone(),
        public_url: public_url.clone(),
        http,
    };
    let backend = state.store.backend();

    let dist = root.join("web/dist");
    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/exercises", get(list_exercises))
        .route("/api/run", post(run_handler))
        .route("/api/me", get(auth::me))
        .route("/api/progress", get(auth::get_progress))
        .route("/api/progress/import", post(auth::import_progress))
        .route("/api/progress/{id}", put(auth::put_progress))
        .route("/auth/github/login", get(auth::login))
        .route("/auth/github/callback", get(auth::callback))
        .route("/auth/logout", post(auth::logout))
        .fallback_service(ServeDir::new(dist))
        .with_state(state);

    let addr = std::net::SocketAddr::from((host, port));
    println!("Make It Real · mir-server en http://{addr} (raíz: {})", root.display());
    println!("Progreso guardado en: {backend}");
    match &github {
        Some(_) => println!("Login con GitHub: activado (callback {}/auth/github/callback)", public_url.as_deref().unwrap_or("<URL de la petición>")),
        None => println!("Login con GitHub: desactivado (falta MIR_GITHUB_CLIENT_ID / MIR_GITHUB_CLIENT_SECRET)"),
    }
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
