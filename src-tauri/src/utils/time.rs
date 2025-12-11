use chrono::{Local, NaiveTime, Timelike};

/// 解析时间字符串 "HH:MM"
pub fn parse_time(time_str: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(time_str, "%H:%M").ok()
}

/// 获取当前时间的 "HH:MM" 格式
pub fn current_time_str() -> String {
    Local::now().format("%H:%M").to_string()
}

/// 计算距离下一个指定时间还有多少分钟
pub fn minutes_until(time_str: &str) -> Option<i64> {
    let target = parse_time(time_str)?;
    let now = Local::now().time();

    let target_minutes = target.num_seconds_from_midnight() as i64 / 60;
    let now_minutes = now.num_seconds_from_midnight() as i64 / 60;

    let diff = target_minutes - now_minutes;

    if diff > 0 {
        Some(diff)
    } else {
        // 如果已经过了今天的时间，计算到明天的时间
        Some(diff + 24 * 60)
    }
}
