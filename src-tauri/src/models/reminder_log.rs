use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::reminder::SyncStatus;

/// 日志操作类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LogAction {
    Triggered,
    Completed,
    Dismissed,
    Snoozed,
}

impl LogAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Triggered => "triggered",
            Self::Completed => "completed",
            Self::Dismissed => "dismissed",
            Self::Snoozed => "snoozed",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "completed" => Self::Completed,
            "dismissed" => Self::Dismissed,
            "snoozed" => Self::Snoozed,
            _ => Self::Triggered,
        }
    }
}

/// 提醒日志
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderLog {
    pub id: Option<i64>,
    pub uuid: String,
    pub reminder_uuid: String,
    pub action: LogAction,
    pub triggered_at: DateTime<Utc>,
    pub action_at: Option<DateTime<Utc>>,
    pub snooze_until: Option<DateTime<Utc>>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub sync_status: SyncStatus,
    pub sync_at: Option<DateTime<Utc>>,
}
