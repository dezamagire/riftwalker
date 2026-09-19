mod appbar;
mod desktops;
mod hotkeys;

use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW,
    SetWindowLongW,
    GWL_EXSTYLE,
    WS_EX_APPWINDOW,
    WS_EX_NOACTIVATE,
    WS_EX_TOOLWINDOW,
    WS_EX_TOPMOST,
};
use raw_window_handle::{
    HasWindowHandle,
    RawWindowHandle,
};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .unwrap();

            window.set_position(
                tauri::Position::Physical(
                    tauri::PhysicalPosition {
                        x: 0,
                        y: 0,
                    },
                ),
            )?;

            window.set_always_on_top(true)?;

            let handle =
                window.window_handle()?;

            if let RawWindowHandle::Win32(handle) =
                handle.as_raw()
            {
                let hwnd =
                    windows::Win32::Foundation::HWND(
                        handle.hwnd.get()
                            as *mut std::ffi::c_void,
                    );

                appbar::register(
                    hwnd,
                    1920,
                    40,
                )?;
                
                desktops::init(hwnd)?;

                // Pin the specific window.
                desktops::pin_window(hwnd)?;

                // // Pin the entire application by AppUserModelId.
                // desktops::pin_app(hwnd)?;

                unsafe {
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

                    let new_style =
                        (ex_style & !(WS_EX_APPWINDOW.0 as i32))
                        | (WS_EX_TOOLWINDOW.0 as i32)
                        | (WS_EX_NOACTIVATE.0 as i32)
                        | (WS_EX_TOPMOST.0 as i32);

                    SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);
                }
            }

            hotkeys::start(
                app.handle().clone(),
            );

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect(
            "error while running Riftwalker",
        );
}