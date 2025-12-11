use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{interval, Duration};
use tokio::sync::Mutex;
use chrono::{Local, Timelike, Datelike, DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;

use crate::storage::{reminder_repo, reminder_log_repo, settings_repo};
use crate::models::reminder::{Reminder, RemindType};
use crate::models::reminder_log::LogAction;
use crate::notification::{serverchan, popup, tray};

/// 延迟提醒项
#[derive(Debug, Clone)]
pub struct SnoozeItem {
    pub reminder_uuid: String,
    pub snooze_until: DateTime<Utc>,
    pub original_triggered_at: DateTime<Utc>,
}

/// 循环提醒项
#[derive(Debug, Clone)]
pub struct LoopItem {
    pub reminder_uuid: String,
    pub next_loop_at: DateTime<Utc>,
    pub original_triggered_at: DateTime<Utc>,
}

/// 全局延迟队列
static SNOOZE_QUEUE: Lazy<Arc<Mutex<HashMap<String, SnoozeItem>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// 全局循环提醒队列
static LOOP_QUEUE: Lazy<Arc<Mutex<HashMap<String, LoopItem>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// 全局待超时检测队列（已触发但未完成的提醒）
static TIMEOUT_QUEUE: Lazy<Arc<Mutex<HashMap<String, (DateTime<Utc>, i32)>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// 添加延迟提醒到队列
pub async fn add_snooze(reminder_uuid: &str, minutes: i32, triggered_at: DateTime<Utc>) {
    let snooze_until = Utc::now() + chrono::Duration::minutes(minutes as i64);
    println!("[调度器] 添加延迟提醒: {} 将在 {} 分钟后 ({}) 触发",
        reminder_uuid, minutes, snooze_until.with_timezone(&chrono::Local).format("%H:%M:%S"));
    let item = SnoozeItem {
        reminder_uuid: reminder_uuid.to_string(),
        snooze_until,
        original_triggered_at: triggered_at,
    };
    let mut queue = SNOOZE_QUEUE.lock().await;
    queue.insert(reminder_uuid.to_string(), item);
    println!("[调度器] 延迟队列当前大小: {}", queue.len());
}

/// 从延迟队列移除
pub async fn remove_snooze(reminder_uuid: &str) {
    let mut queue = SNOOZE_QUEUE.lock().await;
    queue.remove(reminder_uuid);
}

/// 添加循环提醒到队列
pub async fn add_loop(reminder_uuid: &str, interval_minutes: i32, triggered_at: DateTime<Utc>) {
    let next_loop_at = Utc::now() + chrono::Duration::minutes(interval_minutes as i64);
    let item = LoopItem {
        reminder_uuid: reminder_uuid.to_string(),
        next_loop_at,
        original_triggered_at: triggered_at,
    };
    let mut queue = LOOP_QUEUE.lock().await;
    queue.insert(reminder_uuid.to_string(), item);
}

/// 从循环队列移除（完成时调用）
pub async fn remove_loop(reminder_uuid: &str) {
    let mut queue = LOOP_QUEUE.lock().await;
    queue.remove(reminder_uuid);
}

/// 添加超时检测
pub async fn add_timeout_check(reminder_uuid: &str, timeout_minutes: i32) {
    let mut queue = TIMEOUT_QUEUE.lock().await;
    queue.insert(reminder_uuid.to_string(), (Utc::now(), timeout_minutes));
}

/// 从超时队列移除
pub async fn remove_timeout_check(reminder_uuid: &str) {
    let mut queue = TIMEOUT_QUEUE.lock().await;
    queue.remove(reminder_uuid);
}

/// 启动调度器
pub async fn start_scheduler(app: AppHandle) {
    println!("[调度器] 启动中...");

    // 等待到下一分钟的整点
    let now = Local::now();
    let wait_secs = 60 - now.second() as u64;
    println!("[调度器] 当前时间: {}, 等待 {} 秒后开始检查", now.format("%H:%M:%S"), wait_secs);

    if wait_secs > 0 && wait_secs < 60 {
        tokio::time::sleep(Duration::from_secs(wait_secs)).await;
    }

    println!("[调度器] 开始每分钟检查提醒");

    // 每分钟检查一次
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;

        let check_time = Local::now();
        println!("[调度器] {} 检查提醒...", check_time.format("%H:%M:%S"));

        // 检查常规提醒
        check_reminders(&app).await;

        // 检查延迟队列
        check_snooze_queue(&app).await;

        // 检查循环队列
        check_loop_queue(&app).await;

        // 检查超时
        check_timeout_queue(&app).await;
    }
}

/// 检查常规提醒
async fn check_reminders(app: &AppHandle) {
    let now = Local::now();
    let current_time = now.format("%H:%M").to_string();
    let current_weekday = now.weekday().num_days_from_monday() as u8 + 1;
    let current_day = now.day() as u8;
    let today = now.format("%Y-%m-%d").to_string();

    // 获取当月最后一天
    let last_day_of_month = get_last_day_of_month(now.year(), now.month());

    let reminders = match reminder_repo::get_enabled_reminders() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("[调度器] 获取提醒失败: {}", e);
            return;
        }
    };

    println!("[调度器] 当前时间: {}, 找到 {} 个启用的提醒", current_time, reminders.len());

    for reminder in reminders {
        println!("[调度器]   - {} (设定时间: {})", reminder.title, reminder.remind_time);

        if reminder.remind_time != current_time {
            continue;
        }

        println!("[调度器]   >>> 时间匹配！检查是否今天已触发...");

        // 检查今天是否已经触发过（避免重复触发）
        if let Some(ref last_triggered) = reminder.last_triggered_at {
            let last_triggered_date = last_triggered.with_timezone(&chrono::Local).format("%Y-%m-%d").to_string();
            if last_triggered_date == today {
                println!("[调度器]   >>> 今天已触发过，跳过");
                continue;
            }
        }

        let should_trigger = match reminder.remind_type {
            RemindType::Daily => true,
            RemindType::Once => reminder.last_triggered_at.is_none(),
            RemindType::Weekday => {
                reminder.weekdays
                    .as_ref()
                    .map(|days| days.contains(&current_weekday))
                    .unwrap_or(false)
            }
            RemindType::Monthly => {
                reminder.monthdays
                    .as_ref()
                    .map(|days| {
                        // 直接匹配当前日期
                        if days.contains(&current_day) {
                            return true;
                        }
                        // 如果今天是当月最后一天，检查是否有设置超过当月天数的日期
                        // 例如：设置了31日，但2月只有28天，则在28日触发
                        if current_day == last_day_of_month {
                            days.iter().any(|&d| d > last_day_of_month)
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            }
        };

        if should_trigger {
            println!("[调度器]   >>> 触发提醒: {}", reminder.title);
            trigger_reminder(app, &reminder).await;
        }
    }
}

/// 获取指定年月的最后一天
fn get_last_day_of_month(year: i32, month: u32) -> u8 {
    use chrono::NaiveDate;

    // 获取下个月的第一天，然后减一天
    let next_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    };

    next_month
        .and_then(|d| d.pred_opt())
        .map(|d| d.day() as u8)
        .unwrap_or(28) // fallback
}

/// 检查延迟队列
async fn check_snooze_queue(app: &AppHandle) {
    let now = Utc::now();
    let mut to_trigger = vec![];

    {
        let queue = SNOOZE_QUEUE.lock().await;
        if !queue.is_empty() {
            println!("[调度器] 延迟队列中有 {} 个项目", queue.len());
            for (uuid, item) in queue.iter() {
                let remaining = item.snooze_until.signed_duration_since(now);
                println!("[调度器]   - {} 还剩 {} 秒", uuid, remaining.num_seconds());
            }
        }
    }

    {
        let mut queue = SNOOZE_QUEUE.lock().await;
        let expired: Vec<String> = queue.iter()
            .filter(|(_, item)| item.snooze_until <= now)
            .map(|(k, _)| k.clone())
            .collect();

        for uuid in expired {
            if let Some(item) = queue.remove(&uuid) {
                println!("[调度器] 延迟提醒到期: {}", uuid);
                to_trigger.push(item);
            }
        }
    }

    for item in to_trigger {
        if let Ok(Some(reminder)) = reminder_repo::get_by_uuid(&item.reminder_uuid) {
            if reminder.is_enabled {
                println!("[调度器] 触发延迟提醒: {}", reminder.title);
                trigger_reminder_internal(app, &reminder, true).await;
            } else {
                println!("[调度器] 延迟提醒已禁用，跳过: {}", reminder.title);
            }
        } else {
            println!("[调度器] 延迟提醒未找到: {}", item.reminder_uuid);
        }
    }
}

/// 检查循环队列
async fn check_loop_queue(app: &AppHandle) {
    let now = Utc::now();
    let mut to_trigger = vec![];

    {
        let mut queue = LOOP_QUEUE.lock().await;
        let expired: Vec<String> = queue.iter()
            .filter(|(_, item)| item.next_loop_at <= now)
            .map(|(k, _)| k.clone())
            .collect();

        for uuid in expired {
            if let Some(item) = queue.remove(&uuid) {
                to_trigger.push(item);
            }
        }
    }

    for item in to_trigger {
        if let Ok(Some(reminder)) = reminder_repo::get_by_uuid(&item.reminder_uuid) {
            if reminder.is_enabled {
                // 循环提醒继续触发
                trigger_reminder_internal(app, &reminder, true).await;

                // 如果还是循环模式，重新加入队列
                if reminder.is_loop {
                    if let Some(interval) = reminder.loop_interval {
                        add_loop(&reminder.uuid, interval, item.original_triggered_at).await;
                    }
                }
            }
        }
    }
}

/// 检查超时队列
async fn check_timeout_queue(app: &AppHandle) {
    let now = Utc::now();
    let mut timed_out = vec![];

    {
        let mut queue = TIMEOUT_QUEUE.lock().await;
        let expired: Vec<String> = queue.iter()
            .filter(|(_, (triggered_at, timeout_minutes))| {
                let deadline = *triggered_at + chrono::Duration::minutes(*timeout_minutes as i64);
                deadline <= now
            })
            .map(|(k, _)| k.clone())
            .collect();

        for uuid in expired {
            queue.remove(&uuid);
            timed_out.push(uuid);
        }
    }

    for uuid in timed_out {
        if let Ok(Some(reminder)) = reminder_repo::get_by_uuid(&uuid) {
            handle_timeout(app, &reminder).await;
        }
    }
}

/// 处理超时
async fn handle_timeout(app: &AppHandle, reminder: &Reminder) {
    // 发送超时事件到前端
    let _ = app.emit("reminder-timeout", reminder.clone());

    // 发送 Server酱通知
    if reminder.notify_on_timeout {
        send_serverchan_notification(
            &format!("提醒超时: {}", reminder.title),
            &format!("提醒「{}」已超时未完成", reminder.title),
        ).await;
    }
}

/// 触发提醒
async fn trigger_reminder(app: &AppHandle, reminder: &Reminder) {
    trigger_reminder_internal(app, reminder, false).await;
}

/// 触发提醒内部实现
async fn trigger_reminder_internal(app: &AppHandle, reminder: &Reminder, is_repeat: bool) {
    let now = Utc::now();

    // 打印日志，确认任务被触发
    println!("========================================");
    println!("[提醒触发] {} - {}", Local::now().format("%Y-%m-%d %H:%M:%S"), reminder.title);
    println!("========================================");

    // 托盘图标闪动
    tray::start_tray_flash(app).await;

    // 更新触发时间
    if !is_repeat {
        if let Err(e) = reminder_repo::update_triggered_at(&reminder.uuid) {
            eprintln!("Failed to update triggered_at: {}", e);
        }
    }

    // 记录触发日志
    if let Err(e) = reminder_log_repo::create_triggered(
        &reminder.uuid,
        if is_repeat { Some("重复提醒".to_string()) } else { None },
    ) {
        eprintln!("Failed to create trigger log: {}", e);
    }

    // 显示独立弹窗（在屏幕右下角/右上角）
    println!("[提醒触发] 正在创建弹窗窗口: {}", reminder.uuid);
    match popup::show_reminder_popup(app, &reminder.uuid) {
        Ok(_) => println!("[提醒触发] 弹窗创建成功"),
        Err(e) => {
            eprintln!("[提醒触发] 弹窗创建失败: {}", e);
            // 如果弹窗失败，回退到显示主窗口
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    }

    // 如果有超时设置，加入超时检测队列
    if let Some(timeout_minutes) = reminder.timeout_minutes {
        if timeout_minutes > 0 && reminder.notify_on_timeout {
            add_timeout_check(&reminder.uuid, timeout_minutes).await;
        }
    }

    // 如果是循环提醒，加入循环队列
    if reminder.is_loop {
        if let Some(interval) = reminder.loop_interval {
            add_loop(&reminder.uuid, interval, now).await;
        }
    }

    // 发送 Server酱通知
    if reminder.notify_on_trigger {
        send_serverchan_notification(
            &format!("提醒触发: {}", reminder.title),
            &reminder.content,
        ).await;
    }
}

/// 发送 Server酱通知
async fn send_serverchan_notification(title: &str, content: &str) {
    let (domain, sendkey) = match settings_repo::get_all() {
        Ok(settings) => (settings.serverchan_domain, settings.serverchan_sendkey),
        Err(_) => (None, None),
    };

    if let Some(key) = sendkey {
        if !key.is_empty() {
            if let Err(e) = serverchan::send_notification(domain.as_deref(), &key, title, content).await {
                eprintln!("Failed to send Server酱 notification: {}", e);
            }
        }
    }
}

/// 完成提醒（供外部调用）
pub async fn complete_reminder_action(app: &AppHandle, uuid: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reminder = reminder_repo::get_by_uuid(uuid)?
        .ok_or("Reminder not found")?;

    // 更新完成时间
    reminder_repo::update_completed_at(uuid)?;

    // 记录完成日志
    reminder_log_repo::create(
        uuid,
        LogAction::Completed,
        Utc::now(),
        None,
        None,
    )?;

    // 从各种队列中移除
    remove_loop(uuid).await;
    remove_snooze(uuid).await;
    remove_timeout_check(uuid).await;

    // 发送完成通知
    if reminder.notify_on_complete {
        send_serverchan_notification(
            &format!("提醒完成: {}", reminder.title),
            &format!("提醒「{}」已完成", reminder.title),
        ).await;
    }

    // 发送完成事件到前端
    let _ = app.emit("reminder-completed", uuid.to_string());

    Ok(())
}

/// 延迟提醒（供外部调用）
pub async fn snooze_reminder_action(
    _app: &AppHandle,
    uuid: &str,
    minutes: i32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let now = Utc::now();
    let snooze_until = now + chrono::Duration::minutes(minutes as i64);

    // 记录延迟日志
    reminder_log_repo::create(
        uuid,
        LogAction::Snoozed,
        now,
        Some(snooze_until),
        Some(format!("延迟 {} 分钟", minutes)),
    )?;

    // 从循环队列移除（延迟优先）
    remove_loop(uuid).await;

    // 从超时队列移除（重新计时）
    remove_timeout_check(uuid).await;

    // 加入延迟队列
    add_snooze(uuid, minutes, now).await;

    Ok(())
}

/// 关闭提醒（供外部调用）
pub async fn dismiss_reminder_action(
    app: &AppHandle,
    uuid: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reminder = reminder_repo::get_by_uuid(uuid)?
        .ok_or("Reminder not found")?;

    let now = Utc::now();

    // 记录关闭日志
    reminder_log_repo::create(
        uuid,
        LogAction::Dismissed,
        now,
        None,
        None,
    )?;

    // 从延迟队列移除
    remove_snooze(uuid).await;

    // 从超时队列移除
    remove_timeout_check(uuid).await;

    // 处理 repeat_on_close 逻辑
    if reminder.repeat_on_close {
        if let Some(interval) = reminder.repeat_interval {
            // 加入延迟队列，间隔指定时间后再次提醒
            add_snooze(uuid, interval, now).await;
        }
    } else {
        // 如果不是 repeat_on_close，也从循环队列移除
        remove_loop(uuid).await;
    }

    // 发送关闭事件到前端
    let _ = app.emit("reminder-dismissed", uuid.to_string());

    Ok(())
}
