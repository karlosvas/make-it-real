//! Login con GitHub (OAuth) y API de progreso del usuario.

use crate::AppState;
use crate::db::{SESSION_DAYS, User, random_token};
use crate::exercise;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{Html, IntoResponse, Json, Redirect, Response};
use serde::Deserialize;
use std::collections::HashMap;

const SESSION_COOKIE: &str = "mir_session";
const STATE_COOKIE: &str = "mir_oauth";
const MAX_FILE_BYTES: usize = 40 * 1024;

#[derive(Clone)]
pub struct GithubConfig {
    pub client_id: String,
    pub client_secret: String,
}

pub fn cookie(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|s| s.split(';'))
        .filter_map(|kv| kv.trim().split_once('='))
        .find(|(k, _)| *k == name)
        .map(|(_, v)| v.to_string())
}

fn set_cookie(res: &mut Response, value: String) {
    if let Ok(v) = HeaderValue::from_str(&value) {
        res.headers_mut().append(header::SET_COOKIE, v);
    }
}

pub async fn current_user(state: &AppState, headers: &HeaderMap) -> Option<User> {
    let token = cookie(headers, SESSION_COOKIE)?;
    state.store.session_user(&token).await.ok().flatten()
}

/// URL pública de la web: MIR_PUBLIC_URL si está definida; si no, la del propio request.
pub fn public_base(state: &AppState, headers: &HeaderMap) -> String {
    if let Some(u) = &state.public_url {
        return u.trim_end_matches('/').to_string();
    }
    let header_str = |name: &str| headers.get(name).and_then(|v| v.to_str().ok());
    let host = header_str("x-forwarded-host").or_else(|| header_str("host")).unwrap_or("127.0.0.1:8787");
    let local = host.starts_with("127.") || host.starts_with("localhost");
    let proto = header_str("x-forwarded-proto").unwrap_or(if local { "http" } else { "https" });
    format!("{proto}://{host}")
}

fn cookie_attrs(base: &str) -> &'static str {
    if base.starts_with("https://") { "Path=/; HttpOnly; SameSite=Lax; Secure" } else { "Path=/; HttpOnly; SameSite=Lax" }
}

fn safe_next(next: Option<&str>) -> String {
    match next {
        Some(n) if n.starts_with('/') && !n.starts_with("//") && !n.contains('\\') => n.to_string(),
        _ => "/".to_string(),
    }
}

fn not_configured() -> Response {
    Html(
        "<meta charset='utf-8'><body style='font-family:sans-serif;background:#0b0d12;color:#e8ebf2;padding:40px'>\
         <h2>El login con GitHub no está configurado</h2>\
         <p>Crea un archivo <code>.env</code> en la raíz del repo con <code>MIR_GITHUB_CLIENT_ID</code> y \
         <code>MIR_GITHUB_CLIENT_SECRET</code> (ver <code>.env.example</code>) y reinicia mir-server.</p>\
         <p><a style='color:#ff8f4d' href='/'>Volver</a></p></body>",
    )
    .into_response()
}

#[derive(Deserialize)]
pub struct LoginQuery {
    next: Option<String>,
}

pub async fn login(State(state): State<AppState>, headers: HeaderMap, Query(q): Query<LoginQuery>) -> Response {
    let Some(gh) = &state.github else { return not_configured() };
    let base = public_base(&state, &headers);
    let oauth_state = random_token();
    let next = safe_next(q.next.as_deref());
    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&state={}&allow_signup=true",
        urlencode(&gh.client_id),
        urlencode(&redirect_uri(&base)),
        oauth_state
    );
    let mut res = Redirect::to(&url).into_response();
    set_cookie(&mut res, format!("{STATE_COOKIE}={oauth_state}|{}; {}; Max-Age=600", urlencode(&next), cookie_attrs(&base)));
    res
}

fn redirect_uri(base: &str) -> String {
    format!("{base}/auth/github/callback")
}

fn urlencode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

fn urldecode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Some(v) = std::str::from_utf8(&bytes[i + 1..i + 3]).ok().and_then(|h| u8::from_str_radix(h, 16).ok()) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    error_description: Option<String>,
}

#[derive(Deserialize)]
struct GithubUser {
    id: i64,
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
}

fn auth_error(msg: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Html(format!(
            "<meta charset='utf-8'><body style='font-family:sans-serif;background:#0b0d12;color:#e8ebf2;padding:40px'>\
             <h2>No se pudo iniciar sesión con GitHub</h2><p>{}</p><p><a style='color:#ff8f4d' href='/'>Volver</a></p></body>",
            msg.replace('<', "&lt;")
        )),
    )
        .into_response()
}

pub async fn callback(State(state): State<AppState>, headers: HeaderMap, Query(q): Query<CallbackQuery>) -> Response {
    let Some(gh) = state.github.clone() else { return not_configured() };
    if let Some(e) = q.error {
        return auth_error(&format!("GitHub ha devuelto: {e}"));
    }
    let (Some(code), Some(got_state)) = (q.code, q.state) else { return auth_error("Faltan parámetros en la respuesta de GitHub.") };
    let Some(saved) = cookie(&headers, STATE_COOKIE) else { return auth_error("La sesión de login ha caducado. Vuelve a intentarlo.") };
    let (expected_state, next) = saved.split_once('|').unwrap_or((saved.as_str(), "%2F"));
    if expected_state != got_state {
        return auth_error("El parámetro state no coincide. Vuelve a intentarlo.");
    }
    let next = safe_next(Some(&urldecode(next)));
    let base = public_base(&state, &headers);

    let token: TokenResponse = match state
        .http
        .post("https://github.com/login/oauth/access_token")
        .header(header::ACCEPT, "application/json")
        .form(&[("client_id", gh.client_id.as_str()), ("client_secret", gh.client_secret.as_str()), ("code", code.as_str()), ("redirect_uri", redirect_uri(&base).as_str())])
        .send()
        .await
    {
        Ok(r) => match r.json().await {
            Ok(t) => t,
            Err(e) => return auth_error(&format!("Respuesta inesperada de GitHub: {e}")),
        },
        Err(e) => return auth_error(&format!("No se pudo contactar con GitHub: {e}")),
    };
    let Some(access_token) = token.access_token else {
        return auth_error(&token.error_description.unwrap_or_else(|| "GitHub no ha devuelto un token.".into()));
    };
    let user: GithubUser = match state
        .http
        .get("https://api.github.com/user")
        .bearer_auth(&access_token)
        .header(header::USER_AGENT, "make-it-real")
        .header(header::ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .and_then(|r| r.error_for_status())
    {
        Ok(r) => match r.json().await {
            Ok(u) => u,
            Err(e) => return auth_error(&format!("Perfil de GitHub inesperado: {e}")),
        },
        Err(e) => return auth_error(&format!("No se pudo leer tu perfil de GitHub: {e}")),
    };

    let session = match state.store.upsert_user(user.id, &user.login, user.name.as_deref(), user.avatar_url.as_deref()).await {
        Ok(id) => match state.store.create_session(id).await {
            Ok(s) => s,
            Err(e) => return auth_error(&format!("Error de base de datos: {e}")),
        },
        Err(e) => return auth_error(&format!("Error de base de datos: {e}")),
    };
    let attrs = cookie_attrs(&base);
    let mut res = Redirect::to(&next).into_response();
    set_cookie(&mut res, format!("{SESSION_COOKIE}={session}; {attrs}; Max-Age={}", SESSION_DAYS * 86400));
    set_cookie(&mut res, format!("{STATE_COOKIE}=; {attrs}; Max-Age=0"));
    res
}

pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Response {
    if let Some(token) = cookie(&headers, SESSION_COOKIE) {
        let _ = state.store.delete_session(&token).await;
    }
    let base = public_base(&state, &headers);
    let mut res = Json(serde_json::json!({ "ok": true })).into_response();
    set_cookie(&mut res, format!("{SESSION_COOKIE}=; {}; Max-Age=0", cookie_attrs(&base)));
    res
}

pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let user = current_user(&state, &headers).await;
    Json(serde_json::json!({ "configured": state.github.is_some(), "user": user })).into_response()
}

fn unauthorized() -> Response {
    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Inicia sesión con GitHub" }))).into_response()
}

pub async fn get_progress(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let Some(user) = current_user(&state, &headers).await else { return unauthorized() };
    match state.store.progress(user.id).await {
        Ok(p) => Json(serde_json::json!({ "exercises": p })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}

/// Solo guarda archivos editables de ejercicios que existen.
fn clean_files(state: &AppState, exercise_id: &str, files: HashMap<String, String>) -> Option<HashMap<String, String>> {
    let (meta, _) = exercise::load(&state.root, exercise_id).ok()?;
    Some(
        files
            .into_iter()
            .filter(|(name, content)| content.len() <= MAX_FILE_BYTES && meta.files.iter().any(|f| f.editable && &f.name == name))
            .collect(),
    )
}

#[derive(Deserialize)]
pub struct SaveBody {
    files: HashMap<String, String>,
}

pub async fn put_progress(State(state): State<AppState>, headers: HeaderMap, Path(id): Path<String>, Json(body): Json<SaveBody>) -> Response {
    let Some(user) = current_user(&state, &headers).await else { return unauthorized() };
    let Some(files) = clean_files(&state, &id, body.files) else { return (StatusCode::NOT_FOUND, "ejercicio desconocido").into_response() };
    match state.store.save_files(user.id, &id, &files).await {
        Ok(()) => Json(serde_json::json!({ "ok": true })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct ImportItem {
    #[serde(default)]
    solved: bool,
    #[serde(default)]
    files: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct ImportBody {
    exercises: HashMap<String, ImportItem>,
}

pub async fn import_progress(State(state): State<AppState>, headers: HeaderMap, Json(body): Json<ImportBody>) -> Response {
    let Some(user) = current_user(&state, &headers).await else { return unauthorized() };
    let mut imported = 0;
    for (id, item) in body.exercises.into_iter().take(200) {
        let Some(files) = clean_files(&state, &id, item.files) else { continue };
        if state.store.import(user.id, &id, &files, item.solved).await.is_ok() {
            imported += 1;
        }
    }
    Json(serde_json::json!({ "ok": true, "imported": imported })).into_response()
}
