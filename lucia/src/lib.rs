#![feature(str_from_utf16_endian)]

use std::{sync::RwLock, time::Duration};

use lazy_static::lazy_static;
use windows::core::PCSTR;
use windows::Win32::System::Console;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::{Foundation::HINSTANCE, System::LibraryLoader::GetModuleHandleA};

mod interceptor;
mod modules;
mod util;

use crate::modules::{Http, MhyContext, ModuleManager};

unsafe fn thread_func() {
    // Wait until GameAssembly.dll is loaded
    while GetModuleHandleA(PCSTR(b"GameAssembly.dll\0".as_ptr())).is_err() {
        std::thread::sleep(Duration::from_millis(200));
    }

    let base = *util::GAME_ASSEMBLY_BASE;

    //std::thread::sleep(Duration::from_secs(1));

    util::disable_memory_protection();
    //Console::AllocConsole().unwrap();

    println!("Pgr patch\nMade by yoncodes\nTo work with Lucia:");
    println!("Base: {:X}", base);

    let mut module_manager = MODULE_MANAGER.write().unwrap();
    module_manager.enable(MhyContext::<Http>::new(base));

    println!("Successfully initialized!");
}

lazy_static! {
    static ref MODULE_MANAGER: RwLock<ModuleManager> = RwLock::new(ModuleManager::default());
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(|| thread_func());
    }

    true
}
