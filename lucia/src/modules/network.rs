use std::ffi::CString;
use std::sync::LazyLock;

use super::{MhyContext, MhyModule, ModuleType};
use crate::util::{import, read_csharp_string};
use anyhow::Result;
use ilhook::x64::Registers;

const WEB_REQUEST_UTILS_MAKE_INITIAL_URL: usize = 0x43FBCA0;
const SET_REQUEST_HEADER: usize = 0x43F6A90;

static HOST_CSTRING: LazyLock<CString> = LazyLock::new(|| CString::new("127.0.0.1").unwrap());

pub struct Http;

impl MhyModule for MhyContext<Http> {
    unsafe fn init(&mut self) -> Result<()> {
        self.interceptor.attach(
            self.assembly_base + WEB_REQUEST_UTILS_MAKE_INITIAL_URL,
            Http::on_make_initial_url,
        )?;

        self.interceptor.attach(
            self.assembly_base + SET_REQUEST_HEADER,
            Http::on_set_request_header,
        )?;

        Ok(())
    }

    unsafe fn de_init(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::Http
    }
}

import!(il2cpp_string_new(cstr: *const u8) -> usize = 0x4C2420);
//import!(il2cpp_string_new_utf16(text: *const u16, len: i32) -> usize = 0x4C25F0);

impl Http {
    const REDIRECT_SDK: bool = true;
    //const REDIRECT_HTTP: bool = true;

    unsafe extern "win64" fn on_make_initial_url(reg: *mut Registers, _: usize) {
        let original_ptr = (*reg).rcx as usize;
        let original = read_csharp_string(original_ptr);

        if !original.starts_with("http://") && !original.starts_with("https://") {
            return;
        }

        if original.contains("127.0.0.1") {
            return;
        }

        if original.contains("ipv4.icanhazip.com") {
            return;
        }

        // Determine protocol FIRST
        let is_https = original.starts_with("https://");
        let base_url = if is_https {
            "https://127.0.0.1:443"
        } else {
            "http://127.0.0.1:80"
        };

        // Check if we should redirect
        let should_redirect = match original.as_str() {
            s if s.contains(".kurogame.net") || s.contains(".kurogame.com") => Self::REDIRECT_SDK,
            _ => false,
        };

        if !should_redirect {
            return;
        }

        // Build new URL preserving the protocol
        let mut new_url = base_url.to_string();
        original.split('/').skip(3).for_each(|s| {
            new_url.push('/');
            new_url.push_str(s);
        });

        if original.contains("/event") {
            return;
        }

        println!("\n[Redir] Redirected MakeInitialUrl");
        println!("  Original : {}", original);
        println!("  Redirect : {}", new_url);

        let cstr = CString::new(new_url.as_str()).unwrap();
        let new_ptr = il2cpp_string_new(cstr.as_ptr() as *const u8);

        (*reg).rcx = new_ptr as u64;
    }

    unsafe extern "win64" fn on_set_request_header(reg: *mut Registers, _: usize) {
        let key = read_csharp_string((*reg).rdx as usize);
        let value = read_csharp_string((*reg).r8 as usize);

        if key.is_empty() || value.is_empty() {
            return;
        }

        if key.eq_ignore_ascii_case("host") {
            println!("[SetRequestHeader] Rewriting Host: {value} → localhost");

            let new_ptr = il2cpp_string_new(HOST_CSTRING.as_ptr() as *const u8);
            (*reg).r8 = new_ptr as u64;
        } else {
            println!("[SetRequestHeader] {key}: {value}");
        }
    }
}
