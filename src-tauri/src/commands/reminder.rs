use tauri::AppHandle;
use crate::models::reminder::{Reminder, CreateReminderRequest, UpdateReminderRequest};
use crate::models::reminder_log::ReminderLog;
use crate::services::reminder_service;

#[tauri::command]
pub async fn get_reminders(app: AppHandle) -> Result<Vec<Reminder>, String> {
    reminder_service::get_all_reminders(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_reminder(app: AppHandle, uuid: String) -> Result<Option<Reminder>, String> {
    reminder_service::get_reminder_by_uuid(&app, &uuid).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_reminder(app: AppHandle, request: CreateReminderRequest) -> Result<Reminder, String> {
    reminder_service::create_reminder(&app, request).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_reminder(app: AppHandle, request: UpdateReminderRequest) -> Result<Reminder, String> {
    reminder_service::update_reminder(&app, request).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_reminder(app: AppHandle, uuid: String) -> Result<(), String> {
    reminder_service::delete_reminder(&app, &uuid).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_reminder(app: AppHandle, uuid: String) -> Result<Reminder, String> {
    reminder_service::toggle_reminder(&app, &uuid).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reorder_reminders(app: AppHandle, uuids: Vec<String>) -> Result<(), String> {
    reminder_service::reorder_reminders(&app, &uuids).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn complete_reminder(app: AppHandle, uuid: String) -> Result<(), String> {
    reminder_service::complete_reminder(&app, &uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn snooze_reminder(app: AppHandle, uuid: String, minutes: i32) -> Result<(), String> {
    reminder_service::snooze_reminder(&app, &uuid, minutes).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn dismiss_reminder(app: AppHandle, uuid: String) -> Result<(), String> {
    reminder_service::dismiss_reminder(&app, &uuid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_reminder_logs(
    app: AppHandle,
    reminder_uuid: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    limit: Option<i32>,
) -> Result<Vec<ReminderLog>, String> {
    reminder_service::get_reminder_logs(&app, reminder_uuid, start_date, end_date, limit)
        .map_err(|e| e.to_string())
}
