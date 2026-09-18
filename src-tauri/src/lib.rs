mod appbar;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            window.set_position(
                tauri::Position::Physical(
                    tauri::PhysicalPosition { x: 0, y: 0 }
                )
            )?;

            window.set_always_on_top(true)?;

            let handle = window.window_handle()?;

            if let RawWindowHandle::Win32(handle) = handle.as_raw() {
                let hwnd = windows::Win32::Foundation::HWND(
                    handle.hwnd.get() as *mut std::ffi::c_void
                );
                appbar::register(hwnd, 1920, 40)?;
                            window.set_position(
                tauri::Position::Physical(
                    tauri::PhysicalPosition { x: 0, y: 0 }
                )
            )?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}