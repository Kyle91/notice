use rusqlite::{params, Row};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::models::reminder::{
    Reminder, CreateReminderRequest, UpdateReminderRequest,
    RemindType, SyncStatus, ReminderLink,
};
use crate::storage::database::get_connection;

fn row_to_reminder(row: &Row) -> rusqlite::Result<Reminder> {
    let remind_type_str: String = row.get("remind_type")?;
    let sync_status_str: String = row.get("sync_status")?;
    let links_json: Option<String> = row.get("links")?;
    let weekdays_json: Option<String> = row.get("weekdays")?;
    let monthdays_json: Option<String> = row.get("monthdays").unwrap_or(None);

    Ok(Reminder {
        id: Some(row.get("id")?),
        uuid: row.get("uuid")?,
        user_id: row.get("user_id")?,
        title: row.get("title")?,
        content: row.get("content")?,
        links: links_json.and_then(|s| serde_json::from_str(&s).ok()),
        remind_time: row.get("remind_time")?,
        remind_type: RemindType::from_str(&remind_type_str),
        weekdays: weekdays_json.and_then(|s| serde_json::from_str(&s).ok()),
        monthdays: monthdays_json.and_then(|s| serde_json::from_str(&s).ok()),
        is_enabled: row.get::<_, i32>("is_enabled")? == 1,
        repeat_on_close: row.get::<_, i32>("repeat_on_close")? == 1,
        repeat_interval: row.get("repeat_interval")?,
        is_loop: row.get::<_, i32>("is_loop")? == 1,
        loop_interval: row.get("loop_interval")?,
        notify_on_trigger: row.get::<_, i32>("notify_on_trigger")? == 1,
        notify_on_complete: row.get::<_, i32>("notify_on_complete")? == 1,
        notify_on_timeout: row.get::<_, i32>("notify_on_timeout")? == 1,
        timeout_minutes: row.get("timeout_minutes")?,
        last_triggered_at: row.get::<_, Option<String>>("last_triggered_at")?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        last_completed_at: row.get::<_, Option<String>>("last_completed_at")?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        sort_order: row.get("sort_order")?,
        created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>("created_at")?)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>("updated_at")?)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        deleted_at: row.get::<_, Option<String>>("deleted_at")?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        version: row.get("version")?,
        sync_status: SyncStatus::from_str(&sync_status_str),
        sync_at: row.get::<_, Option<String>>("sync_at")?
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
    })
}

pub fn get_all() -> Result<Vec<Reminder>, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT * FROM reminders WHERE deleted_at IS NULL ORDER BY sort_order ASC, created_at DESC"
    )?;

    let reminders = stmt.query_map([], |row| row_to_reminder(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(reminders)
}

pub fn get_by_uuid(uuid: &str) -> Result<Option<Reminder>, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT * FROM reminders WHERE uuid = ? AND deleted_at IS NULL"
    )?;

    let mut rows = stmt.query(params![uuid])?;
    match rows.next()? {
        Some(row) => Ok(Some(row_to_reminder(row)?)),
        None => Ok(None),
    }
}

pub fn create(request: CreateReminderRequest) -> Result<Reminder, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();
    let uuid = Uuid::new_v4().to_string();

    let links_json = request.links.as_ref().map(|l| serde_json::to_string(l).unwrap());
    let weekdays_json = request.weekdays.as_ref().map(|w| serde_json::to_string(w).unwrap());
    let monthdays_json = request.monthdays.as_ref().map(|m| serde_json::to_string(m).unwrap());

    conn.execute(
        "INSERT INTO reminders (
            uuid, title, content, links, remind_time, remind_type, weekdays, monthdays,
            is_enabled, repeat_on_close, repeat_interval, is_loop, loop_interval,
            notify_on_trigger, notify_on_complete, notify_on_timeout, timeout_minutes,
            sort_order, created_at, updated_at, version, sync_status
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?, 1, 'pending')",
        params![
            uuid,
            request.title,
            request.content.unwrap_or_default(),
            links_json,
            request.remind_time,
            request.remind_type.unwrap_or_default().as_str(),
            weekdays_json,
            monthdays_json,
            request.repeat_on_close.unwrap_or(false) as i32,
            request.repeat_interval,
            request.is_loop.unwrap_or(false) as i32,
            request.loop_interval,
            request.notify_on_trigger.unwrap_or(false) as i32,
            request.notify_on_complete.unwrap_or(false) as i32,
            request.notify_on_timeout.unwrap_or(false) as i32,
            request.timeout_minutes,
            now.to_rfc3339(),
            now.to_rfc3339(),
        ],
    )?;

    drop(conn);
    get_by_uuid(&uuid).map(|opt| opt.expect("Just created reminder not found"))
}

pub fn update(request: UpdateReminderRequest) -> Result<Reminder, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();

    // 构建动态更新语句
    let mut updates = vec!["updated_at = ?", "version = version + 1", "sync_status = 'pending'"];
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(now.to_rfc3339())];

    if let Some(ref title) = request.title {
        updates.push("title = ?");
        params_vec.push(Box::new(title.clone()));
    }
    if let Some(ref content) = request.content {
        updates.push("content = ?");
        params_vec.push(Box::new(content.clone()));
    }
    if let Some(ref links) = request.links {
        updates.push("links = ?");
        params_vec.push(Box::new(serde_json::to_string(links).unwrap()));
    }
    if let Some(ref remind_time) = request.remind_time {
        updates.push("remind_time = ?");
        params_vec.push(Box::new(remind_time.clone()));
        // 当修改提醒时间时，清除上次触发时间，以便重新触发提醒
        updates.push("last_triggered_at = NULL");
    }
    if let Some(ref remind_type) = request.remind_type {
        updates.push("remind_type = ?");
        params_vec.push(Box::new(remind_type.as_str().to_string()));
    }
    if let Some(ref weekdays) = request.weekdays {
        updates.push("weekdays = ?");
        params_vec.push(Box::new(serde_json::to_string(weekdays).unwrap()));
    }
    if let Some(ref monthdays) = request.monthdays {
        updates.push("monthdays = ?");
        params_vec.push(Box::new(serde_json::to_string(monthdays).unwrap()));
    }
    if let Some(is_enabled) = request.is_enabled {
        updates.push("is_enabled = ?");
        params_vec.push(Box::new(is_enabled as i32));
    }
    if let Some(repeat_on_close) = request.repeat_on_close {
        updates.push("repeat_on_close = ?");
        params_vec.push(Box::new(repeat_on_close as i32));
    }
    if let Some(repeat_interval) = request.repeat_interval {
        updates.push("repeat_interval = ?");
        params_vec.push(Box::new(repeat_interval));
    }
    if let Some(is_loop) = request.is_loop {
        updates.push("is_loop = ?");
        params_vec.push(Box::new(is_loop as i32));
    }
    if let Some(loop_interval) = request.loop_interval {
        updates.push("loop_interval = ?");
        params_vec.push(Box::new(loop_interval));
    }
    if let Some(notify_on_trigger) = request.notify_on_trigger {
        updates.push("notify_on_trigger = ?");
        params_vec.push(Box::new(notify_on_trigger as i32));
    }
    if let Some(notify_on_complete) = request.notify_on_complete {
        updates.push("notify_on_complete = ?");
        params_vec.push(Box::new(notify_on_complete as i32));
    }
    if let Some(notify_on_timeout) = request.notify_on_timeout {
        updates.push("notify_on_timeout = ?");
        params_vec.push(Box::new(notify_on_timeout as i32));
    }
    if let Some(timeout_minutes) = request.timeout_minutes {
        updates.push("timeout_minutes = ?");
        params_vec.push(Box::new(timeout_minutes));
    }
    if let Some(sort_order) = request.sort_order {
        updates.push("sort_order = ?");
        params_vec.push(Box::new(sort_order));
    }

    params_vec.push(Box::new(request.uuid.clone()));

    let sql = format!(
        "UPDATE reminders SET {} WHERE uuid = ? AND deleted_at IS NULL",
        updates.join(", ")
    );

    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    conn.execute(&sql, params_refs.as_slice())?;

    drop(conn);
    get_by_uuid(&request.uuid).map(|opt| opt.expect("Updated reminder not found"))
}

pub fn soft_delete(uuid: &str) -> Result<(), rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();

    conn.execute(
        "UPDATE reminders SET deleted_at = ?, updated_at = ?, sync_status = 'pending' WHERE uuid = ?",
        params![now.to_rfc3339(), now.to_rfc3339(), uuid],
    )?;

    Ok(())
}

pub fn toggle_enabled(uuid: &str) -> Result<Reminder, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();

    conn.execute(
        "UPDATE reminders SET is_enabled = NOT is_enabled, updated_at = ?, sync_status = 'pending' WHERE uuid = ? AND deleted_at IS NULL",
        params![now.to_rfc3339(), uuid],
    )?;

    drop(conn);
    get_by_uuid(uuid).map(|opt| opt.expect("Toggled reminder not found"))
}

pub fn update_sort_orders(uuids: &[String]) -> Result<(), rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();

    for (index, uuid) in uuids.iter().enumerate() {
        conn.execute(
            "UPDATE reminders SET sort_order = ?, updated_at = ? WHERE uuid = ?",
            params![index as i32, now.to_rfc3339(), uuid],
        )?;
    }

    Ok(())
}

pub fn update_triggered_at(uuid: &str) -> Result<(), rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();

    conn.execute(
        "UPDATE reminders SET last_triggered_at = ?, updated_at = ? WHERE uuid = ?",
        params![now.to_rfc3339(), now.to_rfc3339(), uuid],
    )?;

    Ok(())
}

pub fn update_completed_at(uuid: &str) -> Result<(), rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let now = Utc::now();

    conn.execute(
        "UPDATE reminders SET last_completed_at = ?, updated_at = ? WHERE uuid = ?",
        params![now.to_rfc3339(), now.to_rfc3339(), uuid],
    )?;

    Ok(())
}

pub fn get_enabled_reminders() -> Result<Vec<Reminder>, rusqlite::Error> {
    let conn = get_connection().lock().unwrap();
    let mut stmt = conn.prepare(
        "SELECT * FROM reminders WHERE is_enabled = 1 AND deleted_at IS NULL ORDER BY remind_time ASC"
    )?;

    let reminders = stmt.query_map([], |row| row_to_reminder(row))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(reminders)
}
