pub mod main_window;
pub mod login_dialog;
pub mod register_dialog;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use windows::Win32::Foundation::HWND;
use crate::types::UserSession;

pub static PARENT_HWND: Lazy<Mutex<Option<HWND>>> = Lazy::new(|| Mutex::new(None));
pub static SESSION: Lazy<Mutex<Option<UserSession>>> = Lazy::new(|| Mutex::new(None));
