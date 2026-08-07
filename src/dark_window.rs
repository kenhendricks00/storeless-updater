//! Eliminate the mismatched flash that appears between when Slint creates its
//! native window and when its renderer produces the first frame.
//!
//! Approach: install a thread-local CBT hook before any Slint code runs.
//! HCBT_CREATEWND fires synchronously during `CreateWindowExW`, before the
//! OS paints the window. We:
//!   1. swap the window class background brush to the app's light canvas, so
//!      WM_ERASEBKGND matches the first rendered frame;
//!   2. explicitly use the light DWM title bar to match the client area.
//!
//! Limited to the first window so dialogs / message boxes keep their
//! system look. Only fires for windows created on the thread that called
//! `install` — i.e. the main UI thread. The splash window runs on its own
//! thread and is not affected (it paints itself).

use std::sync::atomic::{AtomicBool, AtomicIsize, Ordering};

use windows::core::w;
use windows::Win32::Foundation::{BOOL, COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Dwm::{
    DwmSetWindowAttribute, DWMWA_BORDER_COLOR, DWMWA_CAPTION_COLOR, DWMWA_TEXT_COLOR,
    DWMWA_USE_IMMERSIVE_DARK_MODE,
};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, FindWindowW, SetClassLongPtrW, SetWindowsHookExW, GCLP_HBRBACKGROUND,
    HCBT_CREATEWND, HHOOK, WH_CBT,
};

// Matches AppWindow.background `#ffffff`. COLORREF byte order is 0x00BBGGRR.
const COLOR_BG: u32 = 0x00ffffff;

static APPLIED: AtomicBool = AtomicBool::new(false);
static INSTALLED: AtomicBool = AtomicBool::new(false);
static WINDOW_BRUSH: AtomicIsize = AtomicIsize::new(0);

/// Install the hook immediately before the AppWindow is created. Idempotent —
/// subsequent calls are no-ops. Must be called *after* any pre-window
/// dialogs (e.g. `prompt_kill_codex_for` on the auto-update path), so the
/// dialog doesn't consume our one-shot APPLIED flag.
pub fn install() {
    if INSTALLED.swap(true, Ordering::SeqCst) {
        return;
    }
    unsafe {
        let brush = CreateSolidBrush(COLORREF(COLOR_BG));
        WINDOW_BRUSH.store(brush.0 as isize, Ordering::SeqCst);

        // Thread-local hook — only fires for windows created on this thread.
        // Returning the HHOOK leaks; the OS reclaims at process exit.
        let _ = SetWindowsHookExW(
            WH_CBT,
            Some(cbt_proc),
            HINSTANCE::default(),
            GetCurrentThreadId(),
        );
    }
}

unsafe extern "system" fn cbt_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HCBT_CREATEWND as i32 && !APPLIED.swap(true, Ordering::SeqCst) {
        let hwnd = HWND(wparam.0 as *mut _);
        if !hwnd.is_invalid() {
            // Class brush: matching light fill for WM_ERASEBKGND. Affects all future
            // windows of the same class in this process — fine, we have
            // one main window per run.
            let brush = WINDOW_BRUSH.load(Ordering::SeqCst);
            if brush != 0 {
                let _ = SetClassLongPtrW(hwnd, GCLP_HBRBACKGROUND, brush);
            }
            apply_light_attributes(hwnd);
        }
    }
    CallNextHookEx(HHOOK::default(), code, wparam, lparam)
}

/// Re-apply the frame palette after Slint has shown the native window. The
/// renderer can update DWM preferences during initialization, so the CBT-hook
/// pass above is repeated once the final top-level window exists.
pub fn refresh() {
    unsafe {
        if let Ok(hwnd) = FindWindowW(None, w!("Storeless Updater")) {
            apply_light_attributes(hwnd);
        }
    }
}

unsafe fn apply_light_attributes(hwnd: HWND) {
    let dark = BOOL::from(false);
    let caption = COLORREF(0x00ff_ffff);
    let text = COLORREF(0x0000_0000);
    let border = COLORREF(0x00e5_e5e5);
    let _ = DwmSetWindowAttribute(
        hwnd,
        DWMWA_USE_IMMERSIVE_DARK_MODE,
        &dark as *const _ as *const _,
        std::mem::size_of::<BOOL>() as u32,
    );
    let _ = DwmSetWindowAttribute(
        hwnd,
        DWMWA_CAPTION_COLOR,
        &caption as *const _ as *const _,
        std::mem::size_of::<COLORREF>() as u32,
    );
    let _ = DwmSetWindowAttribute(
        hwnd,
        DWMWA_TEXT_COLOR,
        &text as *const _ as *const _,
        std::mem::size_of::<COLORREF>() as u32,
    );
    let _ = DwmSetWindowAttribute(
        hwnd,
        DWMWA_BORDER_COLOR,
        &border as *const _ as *const _,
        std::mem::size_of::<COLORREF>() as u32,
    );
}
