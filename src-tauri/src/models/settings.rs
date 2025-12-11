use serde::{Deserialize, Serialize};

/// 窗口位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub serverchan_domain: Option<String>,
    pub serverchan_sendkey: Option<String>,
    pub auto_start: bool,
    pub default_snooze_interval: i32,
    pub window_position: Option<WindowPosition>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            serverchan_domain: None,
            serverchan_sendkey: None,
            auto_start: false,
            default_snooze_interval: 5,
            window_position: None,
        }
    }
}
