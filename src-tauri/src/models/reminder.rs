use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 提醒类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RemindType {
    Daily,
    Once,
    Weekday,
    Monthly,
}

impl Default for RemindType {
    fn default() -> Self {
        Self::Daily
    }
}

impl RemindType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Daily => "daily",
            Self::Once => "once",
            Self::Weekday => "weekday",
            Self::Monthly => "monthly",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "once" => Self::Once,
            "weekday" => Self::Weekday,
            "monthly" => Self::Monthly,
            _ => Self::Daily,
        }
    }
}

/// 同步状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SyncStatus {
    Pending,
    Synced,
    Conflict,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl SyncStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Synced => "synced",
            Self::Conflict => "conflict",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "synced" => Self::Synced,
            "conflict" => Self::Conflict,
            _ => Self::Pending,
        }
    }
}

/// 网站链接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderLink {
    pub name: String,
    pub url: String,
}

impl ReminderLink {
    pub fn validate_links(links: &Option<Vec<ReminderLink>>) -> Result<(), String> {
        if let Some(ref list) = links {
            if list.len() > 3 {
                return Err("链接数量不能超过3个".to_string());
            }
        }
        Ok(())
    }
}

/// 提醒模型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    pub id: Option<i64>,
    pub uuid: String,
    pub user_id: Option<String>,

    // 基本信息
    pub title: String,
    pub content: String,
    pub links: Option<Vec<ReminderLink>>,

    // 时间设置
    pub remind_time: String,
    pub remind_type: RemindType,
    pub weekdays: Option<Vec<u8>>,
    pub monthdays: Option<Vec<u8>>,

    // 状态
    pub is_enabled: bool,

    // 重复设置
    pub repeat_on_close: bool,
    pub repeat_interval: Option<i32>,
    pub is_loop: bool,
    pub loop_interval: Option<i32>,

    // Server酱通知
    pub notify_on_trigger: bool,
    pub notify_on_complete: bool,
    pub notify_on_timeout: bool,
    pub timeout_minutes: Option<i32>,

    // 运行状态
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub last_completed_at: Option<DateTime<Utc>>,

    // 排序
    pub sort_order: i32,

    // 时间戳
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    // 同步相关
    pub version: i32,
    pub sync_status: SyncStatus,
    pub sync_at: Option<DateTime<Utc>>,
}

/// 创建提醒请求
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReminderRequest {
    pub title: String,
    pub content: Option<String>,
    pub links: Option<Vec<ReminderLink>>,
    pub remind_time: String,
    pub remind_type: Option<RemindType>,
    pub weekdays: Option<Vec<u8>>,
    pub monthdays: Option<Vec<u8>>,
    pub repeat_on_close: Option<bool>,
    pub repeat_interval: Option<i32>,
    pub is_loop: Option<bool>,
    pub loop_interval: Option<i32>,
    pub notify_on_trigger: Option<bool>,
    pub notify_on_complete: Option<bool>,
    pub notify_on_timeout: Option<bool>,
    pub timeout_minutes: Option<i32>,
}

/// 更新提醒请求
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReminderRequest {
    pub uuid: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub links: Option<Vec<ReminderLink>>,
    pub remind_time: Option<String>,
    pub remind_type: Option<RemindType>,
    pub weekdays: Option<Vec<u8>>,
    pub monthdays: Option<Vec<u8>>,
    pub is_enabled: Option<bool>,
    pub repeat_on_close: Option<bool>,
    pub repeat_interval: Option<i32>,
    pub is_loop: Option<bool>,
    pub loop_interval: Option<i32>,
    pub notify_on_trigger: Option<bool>,
    pub notify_on_complete: Option<bool>,
    pub notify_on_timeout: Option<bool>,
    pub timeout_minutes: Option<i32>,
    pub sort_order: Option<i32>,
}
