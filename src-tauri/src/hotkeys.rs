use crate::desktops;

use std::sync::OnceLock;
use std::thread;

use tauri::{AppHandle, Emitter};

use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM},
    UI::{
        Input::KeyboardAndMouse::GetAsyncKeyState,
        WindowsAndMessaging::{
            CallNextHookEx,
            DispatchMessageW,
            GetMessageW,
            SetWindowsHookExW,
            TranslateMessage,
            UnhookWindowsHookEx,
            KBDLLHOOKSTRUCT,
            MSG,
            WH_KEYBOARD_LL,
            WM_KEYDOWN,
            WM_KEYUP,
            WM_QUIT,
            WM_SYSKEYDOWN,
            WM_SYSKEYUP,
        },
    },
};

static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

const VK_LWIN: i32 = 0x5B;

pub fn start(app: AppHandle) {
    let _ = APP_HANDLE.set(app.clone());

    thread::spawn(|| {
        unsafe {
            let hook = match SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_proc),
                Some(HINSTANCE::default()),
                0,
            ) {
                Ok(hook) => hook,
                Err(error) => {
                    eprintln!("Failed to install keyboard hook: {error}");
                    return;
                }
            };

            println!("Riftwalker keyboard hook installed");

            let mut msg = MSG::default();

            while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                if msg.message == WM_QUIT {
                    break;
                }

                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            let _ = UnhookWindowsHookEx(hook);

            println!("Riftwalker keyboard hook stopped");
        }
    });
}

unsafe extern "system" fn keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let event = wparam.0 as u32;
    let key = &*(lparam.0 as *const KBDLLHOOKSTRUCT);

    match event {
        WM_KEYDOWN | WM_SYSKEYDOWN => {
            if (0x31..=0x34).contains(&key.vkCode) {
                let win_down = (GetAsyncKeyState(VK_LWIN) as u16 & 0x8000) != 0;

                if win_down {
                    let workspace = (key.vkCode - 0x30) as usize;
                    let index = workspace - 1;

                    println!("Riftwalker hotkey: Win+{}", workspace);

                    thread::spawn(move || {
                        unsafe {
                            let com_result = windows::Win32::System::Com::CoInitializeEx(
                                None,
                                windows::Win32::System::Com::COINIT_APARTMENTTHREADED,
                            );

                            if com_result.is_err() {
                                eprintln!(
                                    "Failed to initialize COM for workspace switch: {com_result:?}"
                                );
                                return;
                            }
                        }

                        match desktops::switch_desktop(index) {
                            Ok(()) => {
                                println!(
                                    "Riftwalker switched to workspace {}",
                                    workspace
                                );

                                if let Some(app) = APP_HANDLE.get() {
                                    if let Err(error) =
                                        app.emit("workspace-hotkey", workspace)
                                    {
                                        eprintln!(
                                            "Failed to emit workspace hotkey: {error}"
                                        );
                                    }
                                }
                            }

                            Err(error) => {
                                eprintln!(
                                    "Failed to switch to workspace {}: {error}",
                                    workspace
                                );
                            }
                        }
                    });

                    return LRESULT(1);
                }
            }
        }

        WM_KEYUP | WM_SYSKEYUP => {}

        _ => {}
    }

    CallNextHookEx(None, code, wparam, lparam)
}