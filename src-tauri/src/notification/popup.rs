use tauri::{AppHandle, Manager, WebviewWindowBuilder, WebviewUrl, PhysicalPosition};

/// 获取当前已有的弹窗数量
fn get_popup_count(app: &AppHandle) -> usize {
    app.webview_windows()
        .keys()
        .filter(|label| label.starts_with("popup-"))
        .count()
}

pub fn show_reminder_popup(app: &AppHandle, reminder_uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let popup_label = format!("popup-{}", reminder_uuid);

    // 检查弹窗是否已存在，如果存在则聚焦
    if let Some(existing) = app.get_webview_window(&popup_label) {
        let _ = existing.set_focus();
        return Ok(());
    }

    // 获取当前弹窗数量，用于计算位置偏移
    let popup_index = get_popup_count(app);

    // 弹窗尺寸
    let popup_width: f64 = 320.0;
    let popup_height: f64 = 200.0;

    // 获取主显示器信息来计算位置
    let position = if let Some(monitor) = app.primary_monitor()? {
        let monitor_size = monitor.size();
        let monitor_position = monitor.position();
        let scale_factor = monitor.scale_factor();

        // 边距
        let margin: i32 = 20;
        // 每个弹窗的垂直偏移
        let offset_per_popup: i32 = ((popup_height + 10.0) * scale_factor) as i32;

        // 计算物理像素尺寸
        let popup_physical_width = (popup_width * scale_factor) as i32;
        let popup_physical_height = (popup_height * scale_factor) as i32;

        #[cfg(target_os = "macos")]
        {
            // macOS: 右上角 (菜单栏在顶部，通知中心在右上角)
            let x = monitor_position.x + monitor_size.width as i32 - popup_physical_width - margin;
            let y = monitor_position.y + margin + 30 + (popup_index as i32 * offset_per_popup);
            Some(PhysicalPosition::new(x, y))
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Windows/Linux: 右下角向上堆叠
            let x = monitor_position.x + monitor_size.width as i32 - popup_physical_width - margin;
            let base_y = monitor_position.y + monitor_size.height as i32 - popup_physical_height - margin - 50;
            let y = base_y - (popup_index as i32 * offset_per_popup);
            Some(PhysicalPosition::new(x, y))
        }
    } else {
        None
    };

    let mut builder = WebviewWindowBuilder::new(
        app,
        &popup_label,
        WebviewUrl::App(format!("/popup?uuid={}", reminder_uuid).into()),
    )
    .title("提醒")
    .inner_size(popup_width, popup_height)
    .resizable(false)
    .always_on_top(true)
    .decorations(true)
    .visible(true)
    .skip_taskbar(false); // 在任务栏显示，方便用户找到

    // 如果计算出了位置，则设置位置；否则居中
    if let Some(pos) = position {
        builder = builder.position(pos.x as f64, pos.y as f64);
    } else {
        builder = builder.center();
    }

    let popup = builder.build()?;

    // 确保窗口显示并获得焦点
    popup.show()?;
    popup.set_focus()?;

    Ok(())
}

pub fn close_reminder_popup(app: &AppHandle, reminder_uuid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let popup_label = format!("popup-{}", reminder_uuid);

    if let Some(window) = app.get_webview_window(&popup_label) {
        window.close()?;
    }

    Ok(())
}
