use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;
use crate::models::reminder::Reminder;
use crate::models::settings::AppSettings;
use crate::storage::{settings_repo, reminder_repo};
use chrono::Local;

pub fn get_settings(_app: &AppHandle) -> Result<AppSettings, Box<dyn std::error::Error>> {
    Ok(settings_repo::get_all()?)
}

pub fn update_settings(app: &AppHandle, settings: AppSettings) -> Result<(), Box<dyn std::error::Error>> {
    // 如果 auto_start 设置改变，同步更新系统自启动状态
    let old_settings = settings_repo::get_all()?;
    if old_settings.auto_start != settings.auto_start {
        set_auto_start_internal(app, settings.auto_start)?;
    }

    Ok(settings_repo::update_all(&settings)?)
}

pub fn set_auto_start(app: &AppHandle, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    // 更新系统自启动
    set_auto_start_internal(app, enabled)?;

    // 更新设置存储
    settings_repo::set("auto_start", &serde_json::to_string(&enabled)?)?;
    Ok(())
}

fn set_auto_start_internal(app: &AppHandle, enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    let autostart_manager = app.autolaunch();

    if enabled {
        autostart_manager.enable().map_err(|e| format!("Failed to enable autostart: {}", e))?;
    } else {
        autostart_manager.disable().map_err(|e| format!("Failed to disable autostart: {}", e))?;
    }

    Ok(())
}

pub fn get_auto_start(app: &AppHandle) -> Result<bool, Box<dyn std::error::Error>> {
    // 优先从系统获取真实状态
    let autostart_manager = app.autolaunch();
    match autostart_manager.is_enabled() {
        Ok(enabled) => Ok(enabled),
        Err(_) => {
            // 如果无法获取系统状态，从设置中读取
            let settings = settings_repo::get_all()?;
            Ok(settings.auto_start)
        }
    }
}

pub fn get_next_reminder(_app: &AppHandle) -> Result<Option<Reminder>, Box<dyn std::error::Error>> {
    let reminders = reminder_repo::get_enabled_reminders()?;
    let current_time = Local::now().format("%H:%M").to_string();

    // 找到下一个要触发的提醒
    let next = reminders.iter()
        .filter(|r| r.remind_time > current_time)
        .min_by_key(|r| &r.remind_time)
        .or_else(|| reminders.first())
        .cloned();

    Ok(next)
}
