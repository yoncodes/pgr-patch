use crate::globals::{GAME_HWND, SDK_INIT_FLAG_1, SDK_INIT_FLAG_2};
use crate::ui;

use std::sync::atomic::Ordering;

#[no_mangle]
pub extern "C" fn kurosdk_login() -> i64 {
    println!("[KRSDK] *** kurosdk_login called ***");

    if !SDK_INIT_FLAG_1.load(Ordering::SeqCst) || !SDK_INIT_FLAG_2.load(Ordering::SeqCst) {
        println!("[KRSDK] ERROR: fail to open login dialog, has not init success");
        return -1;
    }

    let parent_hwnd = GAME_HWND.lock().unwrap().clone();

    unsafe {
        if let Some(hwnd) = parent_hwnd {
            ui::main_window::show_with_parent(hwnd);
        } else {
            ui::main_window::show();
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn kr_sdk_logout() -> u8 {
    println!("[KRSDK] *** Logout called ***");

    // Clear saved session
    {
        let mut session_lock = ui::SESSION.lock().unwrap();
        *session_lock = None;
    }

    0
}
