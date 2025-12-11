-- 提醒表
CREATE TABLE IF NOT EXISTS reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    user_id TEXT,
    title TEXT NOT NULL,
    content TEXT DEFAULT '',
    links TEXT,
    remind_time TEXT NOT NULL,
    remind_type TEXT NOT NULL DEFAULT 'daily',
    weekdays TEXT,
    monthdays TEXT,
    is_enabled INTEGER NOT NULL DEFAULT 1,
    repeat_on_close INTEGER NOT NULL DEFAULT 0,
    repeat_interval INTEGER,
    is_loop INTEGER NOT NULL DEFAULT 0,
    loop_interval INTEGER,
    notify_on_trigger INTEGER NOT NULL DEFAULT 0,
    notify_on_complete INTEGER NOT NULL DEFAULT 0,
    notify_on_timeout INTEGER NOT NULL DEFAULT 0,
    timeout_minutes INTEGER,
    last_triggered_at TEXT,
    last_completed_at TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    version INTEGER NOT NULL DEFAULT 1,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    sync_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_reminders_uuid ON reminders(uuid);
CREATE INDEX IF NOT EXISTS idx_reminders_user_id ON reminders(user_id);
CREATE INDEX IF NOT EXISTS idx_reminders_remind_time ON reminders(remind_time);
CREATE INDEX IF NOT EXISTS idx_reminders_is_enabled ON reminders(is_enabled);
CREATE INDEX IF NOT EXISTS idx_reminders_sync_status ON reminders(sync_status);

-- 提醒日志表
CREATE TABLE IF NOT EXISTS reminder_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT NOT NULL UNIQUE,
    reminder_uuid TEXT NOT NULL,
    action TEXT NOT NULL,
    triggered_at TEXT NOT NULL,
    action_at TEXT,
    snooze_until TEXT,
    note TEXT,
    created_at TEXT NOT NULL,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    sync_at TEXT,
    FOREIGN KEY (reminder_uuid) REFERENCES reminders(uuid)
);

CREATE INDEX IF NOT EXISTS idx_reminder_logs_reminder_uuid ON reminder_logs(reminder_uuid);
CREATE INDEX IF NOT EXISTS idx_reminder_logs_triggered_at ON reminder_logs(triggered_at);
CREATE INDEX IF NOT EXISTS idx_reminder_logs_sync_status ON reminder_logs(sync_status);

-- 设置表
CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    version INTEGER NOT NULL DEFAULT 1,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    sync_at TEXT
);

-- 同步元数据表
CREATE TABLE IF NOT EXISTS sync_meta (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id TEXT NOT NULL UNIQUE,
    last_sync_at TEXT,
    sync_token TEXT,
    user_id TEXT
);
