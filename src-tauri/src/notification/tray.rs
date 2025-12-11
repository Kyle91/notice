use tauri::{AppHandle, Manager, image::Image, tray::TrayIconId};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::time::{interval, Duration};

static IS_FLASHING: AtomicBool = AtomicBool::new(false);

/// 创建缩小版图标（将原图缩小并居中显示）
fn create_scaled_icon(original: &Image<'_>, scale: f32) -> Image<'static> {
    let orig_width = original.width() as usize;
    let orig_height = original.height() as usize;
    let orig_rgba = original.rgba();

    // 新图标大小与原图相同
    let new_width = orig_width;
    let new_height = orig_height;
    let mut pixels = vec![0u8; new_width * new_height * 4];

    // 计算缩小后的尺寸
    let scaled_width = (orig_width as f32 * scale) as usize;
    let scaled_height = (orig_height as f32 * scale) as usize;

    // 计算偏移量使缩小图居中
    let offset_x = (new_width - scaled_width) / 2;
    let offset_y = (new_height - scaled_height) / 2;

    // 使用最近邻插值进行缩放
    for y in 0..scaled_height {
        for x in 0..scaled_width {
            // 计算原图中对应的像素位置
            let src_x = (x as f32 / scale) as usize;
            let src_y = (y as f32 / scale) as usize;

            let src_idx = (src_y * orig_width + src_x) * 4;
            let dst_idx = ((y + offset_y) * new_width + (x + offset_x)) * 4;

            if src_idx + 3 < orig_rgba.len() && dst_idx + 3 < pixels.len() {
                pixels[dst_idx] = orig_rgba[src_idx];         // R
                pixels[dst_idx + 1] = orig_rgba[src_idx + 1]; // G
                pixels[dst_idx + 2] = orig_rgba[src_idx + 2]; // B
                pixels[dst_idx + 3] = orig_rgba[src_idx + 3]; // A
            }
        }
    }

    Image::new_owned(pixels, new_width as u32, new_height as u32)
}

/// 开始托盘图标闪动
pub async fn start_tray_flash(app: &AppHandle) {
    // 如果已经在闪动，不重复启动
    if IS_FLASHING.swap(true, Ordering::SeqCst) {
        return;
    }

    let app_handle = app.clone();

    tauri::async_runtime::spawn(async move {
        let tray_id = TrayIconId::new(crate::TRAY_ID);

        // 获取原始图标
        let original_icon = app_handle.default_window_icon().cloned();

        // 创建缩小版图标用于闪动效果
        let small_icon = original_icon.as_ref().map(|icon| create_scaled_icon(icon, 0.6));

        let mut flash_interval = interval(Duration::from_millis(400));
        let mut show_small = true;
        let mut flash_count = 0;

        // 闪动 10 次 (4秒)
        while flash_count < 10 && IS_FLASHING.load(Ordering::SeqCst) {
            flash_interval.tick().await;

            if let Some(tray) = app_handle.tray_by_id(&tray_id) {
                if show_small {
                    // 显示缩小版图标
                    if let Some(ref icon) = small_icon {
                        let _ = tray.set_icon(Some(icon.clone()));
                    }
                } else {
                    // 恢复原始图标
                    if let Some(ref icon) = original_icon {
                        let _ = tray.set_icon(Some(icon.clone()));
                    }
                }
                show_small = !show_small;
            }

            flash_count += 1;
        }

        // 闪动结束，恢复原始图标
        if let Some(tray) = app_handle.tray_by_id(&tray_id) {
            if let Some(ref icon) = original_icon {
                let _ = tray.set_icon(Some(icon.clone()));
            }
        }

        IS_FLASHING.store(false, Ordering::SeqCst);
    });
}

/// 停止托盘图标闪动
pub fn stop_tray_flash() {
    IS_FLASHING.store(false, Ordering::SeqCst);
}
