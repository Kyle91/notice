use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::OnceCell;
use tauri::{AppHandle, Manager};

static DATABASE: OnceCell<Mutex<Connection>> = OnceCell::new();

pub fn get_db_path(app: &AppHandle) -> PathBuf {
    let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    app_dir.join("notice.db")
}

pub fn init_database(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = get_db_path(app);
    let conn = Connection::open(&db_path)?;

    // 创建表
    conn.execute_batch(include_str!("../../migrations/001_init.sql"))?;

    DATABASE
        .set(Mutex::new(conn))
        .map_err(|_| "Database already initialized")?;

    Ok(())
}

pub fn get_connection() -> &'static Mutex<Connection> {
    DATABASE.get().expect("Database not initialized")
}
