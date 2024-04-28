use serde::Deserialize;
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        SetWindowPos, SWP_ASYNCWINDOWPOS, SWP_NOACTIVATE, SWP_NOCOPYBITS, SWP_NOOWNERZORDER,
        SWP_NOSENDCHANGING, SWP_NOZORDER,
    },
};

use crate::{
    error_handler::{log_if_error, Result},
    seelen::SEELEN,
    windows_api::WindowsApi,
};

#[derive(Deserialize, Debug)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[tauri::command]
pub fn set_window_position(hwnd: isize, rect: Rect) -> Result<(), String> {
    let hwnd = HWND(hwnd);

    if !WindowsApi::is_window(hwnd) {
        return Ok(());
    }

    WindowsApi::unmaximize_window(hwnd);
    let shadow = WindowsApi::shadow_rect(hwnd)?;
    let result = unsafe {
        SetWindowPos(
            hwnd,
            HWND(0),
            rect.left + shadow.left,
            rect.top + shadow.top,
            rect.right + shadow.right + shadow.left.abs(),
            rect.bottom + shadow.bottom + shadow.top.abs(),
            SWP_NOACTIVATE
                | SWP_NOCOPYBITS
                | SWP_NOZORDER
                | SWP_NOOWNERZORDER
                | SWP_ASYNCWINDOWPOS
                | SWP_NOSENDCHANGING,
        )
    };

    log_if_error(result);
    Ok(())
}

#[tauri::command]
pub fn bounce_handle(hwnd: isize) {
    if let Some(wm) = SEELEN.lock().wm_mut() {
        wm.bounce_handle(HWND(hwnd));
    }
}

#[tauri::command]
pub fn complete_window_setup() {
    let mut seelen = SEELEN.lock();
    if let Some(wm) = seelen.wm_mut() {
        log_if_error(wm.complete_window_setup());
    }
}

#[tauri::command]
pub fn request_focus(hwnd: isize) -> Result<(), String> {
    let hwnd = HWND(hwnd);
    log::trace!(
        "Requesting focus on {:?} - {} , {:?}",
        hwnd,
        WindowsApi::get_window_text(hwnd),
        WindowsApi::exe(hwnd)
    );

    let mut seelen = SEELEN.lock();
    if let Some(wm) = seelen.wm_mut() {
        wm.force_focus(hwnd)?;
    }
    Ok(())
}