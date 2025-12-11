pub mod commands;
pub mod models;
pub mod services;
pub mod scheduler;
pub mod notification;
pub mod storage;
pub mod utils;

use tauri::{
    Manager,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    menu::{Menu, MenuItem},
    WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;

/// 托盘图标 ID
pub const TRAY_ID: &str = "main-tray";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .setup(|app| {
            // 初始化数据库
            let app_handle = app.handle().clone();
            storage::database::init_database(&app_handle)?;

            // 启动调度器
            let app_handle_clone = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                scheduler::manager::start_scheduler(app_handle_clone).await;
            });

            // 创建托盘菜单
            let show_item = MenuItem::with_id(app, "show", "显示窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            // 创建系统托盘（带 ID）
            let _tray = TrayIconBuilder::with_id(TRAY_ID)
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("叮咚 - 提醒助手")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // 窗口关闭时隐藏到托盘而不是退出
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            // 提醒相关
            commands::reminder::get_reminders,
            commands::reminder::get_reminder,
            commands::reminder::create_reminder,
            commands::reminder::update_reminder,
            commands::reminder::delete_reminder,
            commands::reminder::toggle_reminder,
            commands::reminder::reorder_reminders,
            commands::reminder::complete_reminder,
            commands::reminder::snooze_reminder,
            commands::reminder::dismiss_reminder,
            commands::reminder::get_reminder_logs,
            // 设置相关
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::settings::test_serverchan,
            commands::settings::set_auto_start,
            commands::settings::get_auto_start,
            commands::settings::get_next_reminder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
