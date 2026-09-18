use std::mem::size_of;

use windows::Win32::{
    Foundation::{HWND, RECT},
    UI::{
        HiDpi::GetDpiForWindow,
        Shell::{
            SHAppBarMessage,
            ABM_NEW,
            ABM_QUERYPOS,
            ABM_SETPOS,
            ABE_TOP,
            APPBARDATA,
        },
        WindowsAndMessaging::{
            GetWindowRect,
            SetWindowPos,
            HWND_TOP,
            SWP_NOACTIVATE,
            SWP_NOZORDER,
        },
    },
};

pub fn register(
    hwnd: HWND,
    width: i32,
    height: i32,
) -> windows::core::Result<()> {
    unsafe {
        let dpi = GetDpiForWindow(hwnd);
        let height = height * dpi as i32 / 96;
        
        let mut appbar = APPBARDATA {
            cbSize: size_of::<APPBARDATA>() as u32,
            hWnd: hwnd,
            uCallbackMessage: 0,
            uEdge: ABE_TOP,
            rc: RECT {
                left: 0,
                top: 0,
                right: width,
                bottom: height,
            },
            lParam: Default::default(),
        };

        SHAppBarMessage(ABM_NEW, &mut appbar);

        SHAppBarMessage(ABM_QUERYPOS, &mut appbar);
        println!(
            "QUERYPOS: left={} top={} right={} bottom={}",
            appbar.rc.left,
            appbar.rc.top,
            appbar.rc.right,
            appbar.rc.bottom
        );

        appbar.rc.top = 0;
        appbar.rc.bottom = height;

        SHAppBarMessage(ABM_SETPOS, &mut appbar);
        
        SetWindowPos(
            hwnd,
            Some(HWND_TOP),
            appbar.rc.left,
            appbar.rc.top,
            appbar.rc.right - appbar.rc.left,
            appbar.rc.bottom - appbar.rc.top,
            SWP_NOACTIVATE | SWP_NOZORDER,
        )?;

        let result = SHAppBarMessage(ABM_SETPOS, &mut appbar);

        println!("SETPOS result: {}", result);

        let mut window_rect = RECT::default();
        GetWindowRect(hwnd, &mut window_rect);

        println!(
            "WINDOW AFTER SETPOS: left={} top={} right={} bottom={}",
            window_rect.left,
            window_rect.top,
            window_rect.right,
            window_rect.bottom
        );
        
        let mut window_rect = RECT::default();

        GetWindowRect(hwnd, &mut window_rect);

        println!(
            "WINDOW: left={} top={} right={} bottom={}",
            window_rect.left,
            window_rect.top,
            window_rect.right,
            window_rect.bottom
        );

        println!(
            "SETPOS: left={} top={} right={} bottom={}",
            appbar.rc.left,
            appbar.rc.top,
            appbar.rc.right,
            appbar.rc.bottom
        );
        let dpi = GetDpiForWindow(hwnd);
        println!("DPI: {}", dpi);
        Ok(())
    }
}