use windows::Win32::{
    Foundation::{CloseHandle, HWND},
    System::{
        ProcessStatus::GetModuleBaseNameW,
        Threading::{
            OpenProcess,
            PROCESS_QUERY_INFORMATION,
            PROCESS_VM_READ,
        },
    },
    UI::WindowsAndMessaging::{
        GetForegroundWindow,
        GetWindowThreadProcessId,
    },
};

pub fn get_foreground_app() -> Option<String> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();

        if hwnd.0.is_null() {
            return None;
        }

        let mut pid = 0u32;

        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        if pid == 0 {
            return None;
        }

        let process = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            pid,
        )
        .ok()?;

        let mut buffer = [0u16; 260];

        let length = GetModuleBaseNameW(
            process,
            None,
            &mut buffer,
        );

        CloseHandle(process).ok();

        if length == 0 {
            return None;
        }

        let name = String::from_utf16_lossy(&buffer[..length as usize]);

        Some(
            name.strip_suffix(".exe")
                .unwrap_or(&name)
                .to_string(),
        )
    }
}