//! Base de datos SQLite: usuarios de GitHub, sesiones y progreso por ejercicio.

use rusqlite::{Connection, OptionalExtension, params};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub const SESSION_DAYS: i64 = 30;

pub struct Db(Mutex<Connection>);

#[derive(Clone, Debug, Serialize)]
pub struct User {
    #[serde(skip)]
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExerciseProgress {
    pub solved: bool,
    pub files: HashMap<String, String>,
    pub updated_at: i64,
}

pub fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

pub fn random_token() -> String {
    let mut buf = [0u8; 32];
    getrandom::fill(&mut buf).expect("sin fuente de aleatoriedad");
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

impl Db {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             CREATE TABLE IF NOT EXISTS users (
                 id INTEGER PRIMARY KEY,
                 github_id INTEGER NOT NULL UNIQUE,
                 login TEXT NOT NULL,
                 name TEXT,
                 avatar_url TEXT,
                 created_at INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS sessions (
                 token TEXT PRIMARY KEY,
                 user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                 expires_at INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS progress (
                 user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                 exercise_id TEXT NOT NULL,
                 files TEXT NOT NULL DEFAULT '{}',
                 solved_at INTEGER,
                 updated_at INTEGER NOT NULL,
                 PRIMARY KEY (user_id, exercise_id)
             );",
        )?;
        Ok(Self(Mutex::new(conn)))
    }

    fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub fn upsert_user(&self, github_id: i64, login: &str, name: Option<&str>, avatar_url: Option<&str>) -> rusqlite::Result<i64> {
        let c = self.conn();
        c.execute(
            "INSERT INTO users (github_id, login, name, avatar_url, created_at) VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(github_id) DO UPDATE SET login = excluded.login, name = excluded.name, avatar_url = excluded.avatar_url",
            params![github_id, login, name, avatar_url, now()],
        )?;
        c.query_row("SELECT id FROM users WHERE github_id = ?1", params![github_id], |r| r.get(0))
    }

    pub fn create_session(&self, user_id: i64) -> rusqlite::Result<String> {
        let token = random_token();
        let c = self.conn();
        c.execute("DELETE FROM sessions WHERE expires_at < ?1", params![now()])?;
        c.execute("INSERT INTO sessions (token, user_id, expires_at) VALUES (?1, ?2, ?3)", params![token, user_id, now() + SESSION_DAYS * 86400])?;
        Ok(token)
    }

    pub fn session_user(&self, token: &str) -> rusqlite::Result<Option<User>> {
        self.conn()
            .query_row(
                "SELECT u.id, u.login, u.name, u.avatar_url FROM sessions s JOIN users u ON u.id = s.user_id
                 WHERE s.token = ?1 AND s.expires_at > ?2",
                params![token, now()],
                |r| Ok(User { id: r.get(0)?, login: r.get(1)?, name: r.get(2)?, avatar_url: r.get(3)? }),
            )
            .optional()
    }

    pub fn delete_session(&self, token: &str) -> rusqlite::Result<()> {
        self.conn().execute("DELETE FROM sessions WHERE token = ?1", params![token])?;
        Ok(())
    }

    pub fn progress(&self, user_id: i64) -> rusqlite::Result<HashMap<String, ExerciseProgress>> {
        let c = self.conn();
        let mut stmt = c.prepare("SELECT exercise_id, files, solved_at, updated_at FROM progress WHERE user_id = ?1")?;
        let rows = stmt.query_map(params![user_id], |r| {
            let files: String = r.get(1)?;
            let solved_at: Option<i64> = r.get(2)?;
            Ok((r.get::<_, String>(0)?, ExerciseProgress { solved: solved_at.is_some(), files: serde_json::from_str(&files).unwrap_or_default(), updated_at: r.get(3)? }))
        })?;
        rows.collect()
    }

    pub fn save_files(&self, user_id: i64, exercise: &str, files: &HashMap<String, String>) -> rusqlite::Result<()> {
        let json = serde_json::to_string(files).unwrap_or_else(|_| "{}".into());
        self.conn().execute(
            "INSERT INTO progress (user_id, exercise_id, files, updated_at) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(user_id, exercise_id) DO UPDATE SET files = excluded.files, updated_at = excluded.updated_at",
            params![user_id, exercise, json, now()],
        )?;
        Ok(())
    }

    pub fn mark_solved(&self, user_id: i64, exercise: &str) -> rusqlite::Result<()> {
        let t = now();
        self.conn().execute(
            "INSERT INTO progress (user_id, exercise_id, solved_at, updated_at) VALUES (?1, ?2, ?3, ?3)
             ON CONFLICT(user_id, exercise_id) DO UPDATE SET solved_at = COALESCE(solved_at, excluded.solved_at)",
            params![user_id, exercise, t],
        )?;
        Ok(())
    }

    /// Sube lo que había en el navegador sin pisar lo que ya está en el servidor.
    pub fn import(&self, user_id: i64, exercise: &str, files: &HashMap<String, String>, solved: bool) -> rusqlite::Result<()> {
        let json = serde_json::to_string(files).unwrap_or_else(|_| "{}".into());
        let t = now();
        let c = self.conn();
        c.execute(
            "INSERT INTO progress (user_id, exercise_id, files, updated_at) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(user_id, exercise_id) DO UPDATE SET files = CASE WHEN progress.files = '{}' THEN excluded.files ELSE progress.files END",
            params![user_id, exercise, json, t],
        )?;
        if solved {
            c.execute("UPDATE progress SET solved_at = COALESCE(solved_at, ?3) WHERE user_id = ?1 AND exercise_id = ?2", params![user_id, exercise, t])?;
        }
        Ok(())
    }
}
