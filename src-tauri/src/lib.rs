mod appbar;
mod desktops;
mod foreground;
mod hotkeys;

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use tauri::Manager;
use tauri::Emitter;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_NOACTIVATE,
    WS_EX_TOOLWINDOW, WS_EX_TOPMOST,
};

use std::thread;
use std::time::Duration;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .expect("main window not found");

            window.set_position(tauri::Position::Physical(
                tauri::PhysicalPosition { x: 0, y: 0 },
            ))?;

            window.set_always_on_top(true)?;

            let handle = window.window_handle()?;

            if let RawWindowHandle::Win32(handle) = handle.as_raw() {
                let hwnd = HWND(handle.hwnd.get() as *mut std::ffi::c_void);

                println!("SETUP: registering appbar");
                appbar::register(hwnd, 1920, 40)?;
                println!("SETUP: appbar registered");

                println!("SETUP: initializing desktops");
                desktops::init(hwnd)?;
                println!("SETUP: desktops initialized");

                println!("SETUP: pinning window");
                desktops::pin_window(hwnd)?;
                println!("SETUP: window pinned");

                unsafe {
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);

                    let new_style = (ex_style & !(WS_EX_APPWINDOW.0 as i32))
                        | (WS_EX_TOOLWINDOW.0 as i32)
                        | (WS_EX_NOACTIVATE.0 as i32)
                        | (WS_EX_TOPMOST.0 as i32);

                    SetWindowLongW(hwnd, GWL_EXSTYLE, new_style);
                }
            }

            println!("SETUP: starting hotkeys");
            hotkeys::start(app.handle().clone());
            println!("SETUP: complete");

            let app_handle = app.handle().clone();

thread::spawn(move || {
    let mut last_app = String::new();

    loop {
        if let Some(app_name) = foreground::get_foreground_app() {
            if app_name != last_app {
                last_app = app_name.clone();

                println!("FOREGROUND APP: {}", app_name);

                if let Err(error) =
                    app_handle.emit("foreground-app", app_name)
                {
                    eprintln!(
                        "Failed to emit foreground app: {error}"
                    );
                }
            }
        }

        thread::sleep(Duration::from_millis(250));
    }
});

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Riftwalker");
}