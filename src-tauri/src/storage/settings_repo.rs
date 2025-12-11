use rusqlite::params;
use chrono::Utc;
use crate::models::settings::AppSettings;
use crate::storage::database::get_connection;

pub fn get_all() -> Result<AppSettings, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let mut settings = AppSettings::default();

    let mut stmt = conn.prepare("SELECT key, value FROM settings")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;

    for row in rows {
        let (key, value) = row?;
        match key.as_str() {
            "serverchan_domain" => {
                settings.serverchan_domain = serde_json::from_str(&value).ok();
            }
            "serverchan_sendkey" => {
                settings.serverchan_sendkey = serde_json::from_str(&value).ok();
            }
            "auto_start" => {
                settings.auto_start = serde_json::from_str(&value).unwrap_or(false);
            }
            "default_snooze_interval" => {
                settings.default_snooze_interval = serde_json::from_str(&value).unwrap_or(5);
            }
            "window_position" => {
                settings.window_position = serde_json::from_str(&value).ok();
            }
            _ => {}
        }
    }

    Ok(settings)
}

pub fn set(key: &str, value: &str) -> Result<(), rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();

    conn.execute(
        "INSERT INTO settings (key, value, updated_at, version, sync_status)
         VALUES (?, ?, ?, 1, 'pending')
         ON CONFLICT(key) DO UPDATE SET
         value = excluded.value,
         updated_at = excluded.updated_at,
         version = version + 1,
         sync_status = 'pending'",
        params![key, value, now.to_rfc3339()],
    )?;

    Ok(())
}

pub fn update_all(settings: &AppSettings) -> Result<(), rusqlite::Error> {
    if let Some(ref domain) = settings.serverchan_domain {
        set("serverchan_domain", &serde_json::to_string(domain).unwrap())?;
    }
    if let Some(ref sendkey) = settings.serverchan_sendkey {
        set("serverchan_sendkey", &serde_json::to_string(sendkey).unwrap())?;
    }
    set("auto_start", &serde_json::to_string(&settings.auto_start).unwrap())?;
    set("default_snooze_interval", &serde_json::to_string(&settings.default_snooze_interval).unwrap())?;
    if let Some(ref pos) = settings.window_position {
        set("window_position", &serde_json::to_string(pos).unwrap())?;
    }

    Ok(())
}
