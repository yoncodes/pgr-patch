use std::{ffi::c_void, sync::LazyLock};

use windows::{
    core::s,
    Win32::System::{
        LibraryLoader::{GetModuleHandleA, GetProcAddress},
        Memory,
    },
};

#[macro_export]
macro_rules! c {
    ($cstr:expr) => {
        unsafe {
            std::ffi::CStr::from_ptr(concat!($cstr, "\0").as_ptr() as *const std::os::raw::c_char)
                .to_bytes_with_nul()
                .as_ptr()
        }
    };
}

macro_rules! import {
    ($name:ident($($arg_name:ident: $arg_type:ty),*) -> $ret_type:ty = $rva:expr) => {
        pub unsafe fn $name($($arg_name: $arg_type,)*) -> $ret_type {
            static PROC: ::std::sync::LazyLock<usize> = ::std::sync::LazyLock::new(|| *crate::util::GAME_ASSEMBLY_BASE + $rva);

            type FuncType = unsafe extern "fastcall" fn($($arg_type,)*) -> $ret_type;
            ::std::mem::transmute::<usize, FuncType>(*PROC)($($arg_name,)*)
        }
    };
}

pub(crate) use import;

pub static GAME_ASSEMBLY_BASE: LazyLock<usize> =
    LazyLock::new(|| unsafe { GetModuleHandleA(s!("GameAssembly.dll")).unwrap().0 as usize });

#[inline]
pub unsafe fn read_csharp_string(s: usize) -> String {
    let str_length = *(s.wrapping_add(16) as *const u32);
    let str_ptr = s.wrapping_add(20) as *const u8;

    String::from_utf16le_lossy(std::slice::from_raw_parts(
        str_ptr,
        (str_length * 2) as usize,
    ))
}

pub unsafe fn disable_memory_protection() {
    let ntdll = GetModuleHandleA(s!("ntdll.dll")).unwrap();
    let proc_addr = GetProcAddress(ntdll, s!("NtProtectVirtualMemory")).unwrap();

    let nt_func = if GetProcAddress(ntdll, s!("wine_get_version")).is_some() {
        GetProcAddress(ntdll, s!("NtPulseEvent")).unwrap()
    } else {
        GetProcAddress(ntdll, s!("NtQuerySection")).unwrap()
    };

    let mut prot = Memory::PAGE_EXECUTE_READWRITE;
    Memory::VirtualProtect(proc_addr as *const usize as *mut c_void, 1, prot, &mut prot).unwrap();

    let routine = nt_func as *mut u32;
    let routine_val = *(routine as *const usize);

    let lower_bits_mask = !(0xFFu64 << 32);
    let lower_bits = routine_val & lower_bits_mask as usize;

    let offset_val = *((routine as usize + 4) as *const u32);
    let upper_bits = ((offset_val as usize).wrapping_sub(1) as usize) << 32;

    let result = lower_bits | upper_bits;

    *(proc_addr as *mut usize) = result;
    Memory::VirtualProtect(proc_addr as *const usize as *mut c_void, 1, prot, &mut prot).unwrap();
}
