use tauri::AppHandle;
use crate::models::reminder::{Reminder, CreateReminderRequest, UpdateReminderRequest, ReminderLink};
use crate::models::reminder_log::ReminderLog;
use crate::storage::{reminder_repo, reminder_log_repo};
use crate::scheduler::manager;
use crate::notification::popup;

pub fn get_all_reminders(_app: &AppHandle) -> Result<Vec<Reminder>, Box<dyn std::error::Error>> {
    Ok(reminder_repo::get_all()?)
}

pub fn get_reminder_by_uuid(_app: &AppHandle, uuid: &str) -> Result<Option<Reminder>, Box<dyn std::error::Error>> {
    Ok(reminder_repo::get_by_uuid(uuid)?)
}

pub fn create_reminder(_app: &AppHandle, request: CreateReminderRequest) -> Result<Reminder, Box<dyn std::error::Error>> {
    // 验证链接数量
    ReminderLink::validate_links(&request.links)?;
    Ok(reminder_repo::create(request)?)
}

pub fn update_reminder(_app: &AppHandle, request: UpdateReminderRequest) -> Result<Reminder, Box<dyn std::error::Error>> {
    // 验证链接数量
    ReminderLink::validate_links(&request.links)?;
    Ok(reminder_repo::update(request)?)
}

pub fn delete_reminder(_app: &AppHandle, uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(reminder_repo::soft_delete(uuid)?)
}

pub fn toggle_reminder(_app: &AppHandle, uuid: &str) -> Result<Reminder, Box<dyn std::error::Error>> {
    Ok(reminder_repo::toggle_enabled(uuid)?)
}

pub fn reorder_reminders(_app: &AppHandle, uuids: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    Ok(reminder_repo::update_sort_orders(uuids)?)
}

pub async fn complete_reminder(app: &AppHandle, uuid: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = manager::complete_reminder_action(app, uuid).await;
    // 关闭弹窗窗口
    let _ = popup::close_reminder_popup(app, uuid);
    result
}

pub async fn snooze_reminder(app: &AppHandle, uuid: &str, minutes: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = manager::snooze_reminder_action(app, uuid, minutes).await;
    // 关闭弹窗窗口
    let _ = popup::close_reminder_popup(app, uuid);
    result
}

pub async fn dismiss_reminder(app: &AppHandle, uuid: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = manager::dismiss_reminder_action(app, uuid).await;
    // 关闭弹窗窗口
    let _ = popup::close_reminder_popup(app, uuid);
    result
}

pub fn get_reminder_logs(
    _app: &AppHandle,
    reminder_uuid: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<ReminderLog>, Box<dyn std::error::Error>> {
    Ok(reminder_log_repo::query(
        reminder_uuid.as_deref(),
        start_date.as_deref(),
        end_date.as_deref(),
        limit,
    )?)
}
