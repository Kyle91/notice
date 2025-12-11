use tauri::AppHandle;
use crate::models::reminder::Reminder;
use crate::models::settings::AppSettings;
use crate::services::settings_service;

#[tauri::command]
pub async fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    settings_service::get_settings(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    settings_service::update_settings(&app, settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn test_serverchan(domain: Option<String>, sendkey: String) -> Result<bool, String> {
    crate::notification::serverchan::test_connection(domain.as_deref(), &sendkey)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_auto_start(app: AppHandle, enabled: bool) -> Result<(), String> {
    settings_service::set_auto_start(&app, enabled).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_auto_start(app: AppHandle) -> Result<bool, String> {
    settings_service::get_auto_start(&app).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_next_reminder(app: AppHandle) -> Result<Option<Reminder>, String> {
    settings_service::get_next_reminder(&app).map_err(|e| e.to_string())
}
