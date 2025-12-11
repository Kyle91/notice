use rusqlite::{params, Row};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::reminder::SyncStatus;
use crate::models::reminder_log::{ReminderLog, LogAction};
use crate::storage::database::get_connection;

fn row_to_log(row: &Row) -> rusqlite::Result<ReminderLog> {
    let action_str: String = row.get("action")?;
    let sync_status_str: String = row.get("sync_status")?;

    Ok(ReminderLog {
        id: Some(row.get("id")?),
        uuid: row.get("uuid")?,
        reminder_uuid: row.get("reminder_uuid")?,
        action: LogAction::from_str(&action_str),
        triggered_at: DateTime::parse_from_rfc3339(&row.get::<_, String>("triggered_at")?)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        action_at: row.get::<_, Option<String>>("action_at")?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        snooze_until: row.get::<_, Option<String>>("snooze_until")?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        note: row.get("note")?,
        created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>("created_at")?)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        sync_status: SyncStatus::from_str(&sync_status_str),
        sync_at: row.get::<_, Option<String>>("sync_at")?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
    })
}

/// 创建日志记录
pub fn create(
    reminder_uuid: &str,
    action: LogAction,
    triggered_at: DateTime<Utc>,
    snooze_until: Option<DateTime<Utc>>,
    note: Option<String>,
) -> Result<ReminderLog, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();
    let uuid = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO reminder_logs (
            uuid, reminder_uuid, action, triggered_at, action_at, snooze_until,
            note, created_at, sync_status
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'pending')",
        params![
            uuid,
            reminder_uuid,
            action.as_str(),
            triggered_at.to_rfc3339(),
            now.to_rfc3339(),
            snooze_until.map(|dt| dt.to_rfc3339()),
            note,
            now.to_rfc3339(),
        ],
    )?;

    drop(conn);
    get_by_uuid(&uuid).map(|opt| opt.expect("Just created log not found"))
}

/// 创建触发日志（不带 action_at）
pub fn create_triggered(reminder_uuid: &str, note: Option<String>) -> Result<ReminderLog, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();
    let uuid = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO reminder_logs (
            uuid, reminder_uuid, action, triggered_at, created_at, note, sync_status
        ) VALUES (?, ?, ?, ?, ?, ?, 'pending')",
        params![
            uuid,
            reminder_uuid,
            LogAction::Triggered.as_str(),
            now.to_rfc3339(),
            now.to_rfc3339(),
            note,
        ],
    )?;

    drop(conn);
    get_by_uuid(&uuid).map(|opt| opt.expect("Just created log not found"))
}

/// 根据 UUID 获取日志
pub fn get_by_uuid(uuid: &str) -> Result<Option<ReminderLog>, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let mut stmt = conn.prepare("SELECT * FROM reminder_logs WHERE uuid = ?")?;

    let mut rows = stmt.query(params![uuid])?;
    match rows.next()? {
        Some(row) => Ok(Some(row_to_log(row)?)),
        None => Ok(None),
    }
}

/// 获取某个提醒的所有日志
pub fn get_by_reminder_uuid(reminder_uuid: &str) -> Result<Vec<ReminderLog>, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT * FROM reminder_logs WHERE reminder_uuid = ? ORDER BY created_at DESC"
    )?;

    let logs = stmt.query_map(params![reminder_uuid], |row| row_to_log(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(logs)
}

/// 查询日志（带过滤条件）
pub fn query(
    reminder_uuid: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
    limit: Option<i32>,
) -> Result<Vec<ReminderLog>, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();

    let mut conditions = vec!["1=1".to_string()];
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![];

    if let Some(uuid) = reminder_uuid {
        conditions.push("reminder_uuid = ?".to_string());
        params_vec.push(Box::new(uuid.to_string()));
    }

    if let Some(start) = start_date {
        conditions.push("triggered_at >= ?".to_string());
        params_vec.push(Box::new(start.to_string()));
    }

    if let Some(end) = end_date {
        conditions.push("triggered_at <= ?".to_string());
        params_vec.push(Box::new(end.to_string()));
    }

    let limit_clause = limit.map(|l| format!(" LIMIT {}", l)).unwrap_or_default();

    let sql = format!(
        "SELECT * FROM reminder_logs WHERE {} ORDER BY triggered_at DESC{}",
        conditions.join(" AND "),
        limit_clause
    );

    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    let logs = stmt.query_map(params_refs.as_slice(), |row| row_to_log(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(logs)
}

/// 获取提醒最近一次触发的日志（未完成的）
pub fn get_latest_triggered(reminder_uuid: &str) -> Result<Option<ReminderLog>, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT * FROM reminder_logs
         WHERE reminder_uuid = ? AND action = 'triggered'
         ORDER BY triggered_at DESC LIMIT 1"
    )?;

    let mut rows = stmt.query(params![reminder_uuid])?;
    match rows.next()? {
        Some(row) => Ok(Some(row_to_log(row)?)),
        None => Ok(None),
    }
}

/// 删除日志
pub fn delete(uuid: &str) -> Result<(), rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    conn.execute("DELETE FROM reminder_logs WHERE uuid = ?", params![uuid])?;
    Ok(())
}

/// 删除提醒的所有日志
pub fn delete_by_reminder_uuid(reminder_uuid: &str) -> Result<(), rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    conn.execute("DELETE FROM reminder_logs WHERE reminder_uuid = ?", params![reminder_uuid])?;
    Ok(())
}

/// 获取今天某个提醒的完成次数
pub fn get_today_completed_count(reminder_uuid: &str) -> Result<i32, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM reminder_logs
         WHERE reminder_uuid = ? AND action = 'completed'
         AND date(triggered_at) = ?",
        params![reminder_uuid, today],
        |row| row.get(0),
    )?;

    Ok(count)
}
