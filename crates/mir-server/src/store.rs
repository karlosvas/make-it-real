//! Almacenamiento de usuarios, sesiones y progreso.
//! - En Vercel: Upstash Redis por su API REST (variables KV_REST_API_URL/TOKEN o UPSTASH_REDIS_REST_URL/TOKEN).
//! - En local: SQLite en data/mir.db.

use crate::db::{Db, ExerciseProgress, SESSION_DAYS, User, now, random_token};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

pub enum Store {
    Sqlite(Db),
    Redis(Redis),
}

pub struct Redis {
    url: String,
    token: String,
    http: reqwest::Client,
}

#[derive(Serialize, Deserialize, Default)]
struct RedisProgress {
    #[serde(default)]
    files: HashMap<String, String>,
    #[serde(default)]
    solved_at: Option<i64>,
    #[serde(default)]
    updated_at: i64,
}

#[derive(Serialize, Deserialize)]
struct RedisUser {
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
}

type R<T> = Result<T, String>;

impl Redis {
    pub fn new(url: String, token: String, http: reqwest::Client) -> Self {
        Self { url: url.trim_end_matches('/').to_string(), token, http }
    }

    async fn cmd(&self, args: Vec<Value>) -> R<Value> {
        let res: Value = self
            .http
            .post(&self.url)
            .bearer_auth(&self.token)
            .json(&args)
            .send()
            .await
            .map_err(|e| format!("redis: {e}"))?
            .json()
            .await
            .map_err(|e| format!("redis: {e}"))?;
        if let Some(err) = res.get("error") {
            return Err(format!("redis: {err}"));
        }
        Ok(res.get("result").cloned().unwrap_or(Value::Null))
    }

    async fn get_progress(&self, user: i64, exercise: &str) -> R<RedisProgress> {
        let v = self.cmd(vec![json!("HGET"), json!(format!("mir:progress:{user}")), json!(exercise)]).await?;
        Ok(v.as_str().and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default())
    }

    async fn put_progress(&self, user: i64, exercise: &str, p: &RedisProgress) -> R<()> {
        let s = serde_json::to_string(p).map_err(|e| e.to_string())?;
        self.cmd(vec![json!("HSET"), json!(format!("mir:progress:{user}")), json!(exercise), json!(s)]).await?;
        Ok(())
    }
}

impl Store {
    pub fn backend(&self) -> &'static str {
        match self {
            Store::Sqlite(_) => "SQLite (data/mir.db)",
            Store::Redis(_) => "Upstash Redis",
        }
    }

    pub async fn upsert_user(&self, github_id: i64, login: &str, name: Option<&str>, avatar_url: Option<&str>) -> R<i64> {
        match self {
            Store::Sqlite(db) => db.upsert_user(github_id, login, name, avatar_url).map_err(|e| e.to_string()),
            Store::Redis(r) => {
                let u = RedisUser { login: login.into(), name: name.map(Into::into), avatar_url: avatar_url.map(Into::into) };
                let s = serde_json::to_string(&u).map_err(|e| e.to_string())?;
                r.cmd(vec![json!("SET"), json!(format!("mir:user:{github_id}")), json!(s)]).await?;
                Ok(github_id)
            }
        }
    }

    pub async fn create_session(&self, user_id: i64) -> R<String> {
        match self {
            Store::Sqlite(db) => db.create_session(user_id).map_err(|e| e.to_string()),
            Store::Redis(r) => {
                let token = random_token();
                r.cmd(vec![json!("SET"), json!(format!("mir:session:{token}")), json!(user_id.to_string()), json!("EX"), json!(SESSION_DAYS * 86400)]).await?;
                Ok(token)
            }
        }
    }

    pub async fn session_user(&self, token: &str) -> R<Option<User>> {
        if token.len() != 64 || !token.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Ok(None);
        }
        match self {
            Store::Sqlite(db) => db.session_user(token).map_err(|e| e.to_string()),
            Store::Redis(r) => {
                let Some(id) = r.cmd(vec![json!("GET"), json!(format!("mir:session:{token}"))]).await?.as_str().and_then(|s| s.parse::<i64>().ok()) else {
                    return Ok(None);
                };
                let user = r.cmd(vec![json!("GET"), json!(format!("mir:user:{id}"))]).await?;
                Ok(user.as_str().and_then(|s| serde_json::from_str::<RedisUser>(s).ok()).map(|u| User { id, login: u.login, name: u.name, avatar_url: u.avatar_url }))
            }
        }
    }

    pub async fn delete_session(&self, token: &str) -> R<()> {
        match self {
            Store::Sqlite(db) => db.delete_session(token).map_err(|e| e.to_string()),
            Store::Redis(r) => r.cmd(vec![json!("DEL"), json!(format!("mir:session:{token}"))]).await.map(|_| ()),
        }
    }

    pub async fn progress(&self, user_id: i64) -> R<HashMap<String, ExerciseProgress>> {
        match self {
            Store::Sqlite(db) => db.progress(user_id).map_err(|e| e.to_string()),
            Store::Redis(r) => {
                let v = r.cmd(vec![json!("HGETALL"), json!(format!("mir:progress:{user_id}"))]).await?;
                let items = v.as_array().cloned().unwrap_or_default();
                let mut out = HashMap::new();
                for pair in items.chunks(2) {
                    if let [Value::String(k), Value::String(val)] = pair {
                        let p: RedisProgress = serde_json::from_str(val).unwrap_or_default();
                        out.insert(k.clone(), ExerciseProgress { solved: p.solved_at.is_some(), files: p.files, updated_at: p.updated_at });
                    }
                }
                Ok(out)
            }
        }
    }

    pub async fn save_files(&self, user_id: i64, exercise: &str, files: &HashMap<String, String>) -> R<()> {
        match self {
            Store::Sqlite(db) => db.save_files(user_id, exercise, files).map_err(|e| e.to_string()),
            Store::Redis(r) => {
                let mut p = r.get_progress(user_id, exercise).await?;
                p.files = files.clone();
                p.updated_at = now();
                r.put_progress(user_id, exercise, &p).await
            }
        }
    }

    pub async fn mark_solved(&self, user_id: i64, exercise: &str) -> R<()> {
        match self {
            Store::Sqlite(db) => db.mark_solved(user_id, exercise).map_err(|e| e.to_string()),
            Store::Redis(r) => {
                let mut p = r.get_progress(user_id, exercise).await?;
                if p.solved_at.is_none() {
                    p.solved_at = Some(now());
                    p.updated_at = now();
                    r.put_progress(user_id, exercise, &p).await?;
                }
                Ok(())
            }
        }
    }

    /// Sube lo que había en el navegador sin pisar lo que ya está guardado.
    pub async fn import(&self, user_id: i64, exercise: &str, files: &HashMap<String, String>, solved: bool) -> R<()> {
        match self {
            Store::Sqlite(db) => db.import(user_id, exercise, files, solved).map_err(|e| e.to_string()),
            Store::Redis(r) => {
                let mut p = r.get_progress(user_id, exercise).await?;
                if p.files.is_empty() {
                    p.files = files.clone();
                }
                if solved && p.solved_at.is_none() {
                    p.solved_at = Some(now());
                }
                p.updated_at = now();
                r.put_progress(user_id, exercise, &p).await
            }
        }
    }
}
