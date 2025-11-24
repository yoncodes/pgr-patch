use crate::globals::{
    AGREEMENT_CSTRING, AGREEMENT_DATA, CALLBACK, CONFIG_CSTRING, DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH, SDK_CONFIG, SDK_INITIALIZED, SDK_INIT_FLAG_1, SDK_INIT_FLAG_2,
};
use std::ffi::{c_char, CString};
use std::os::raw::c_void;
use std::sync::atomic::Ordering;
#[allow(unused_imports)]
use windows::Win32::System::Console::{AllocConsole, FreeConsole};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(
    _hinst_dll: *mut c_void,
    fdw_reason: u32,
    _lpv_reserved: *mut c_void,
) -> i32 {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            #[cfg(windows)]
            #[allow(unused_unsafe)]
            unsafe {
                //let _ = AllocConsole();
                println!("[KRSDK] DLL Loaded - Console initialized");
            }
        }
        DLL_PROCESS_DETACH => {
            #[cfg(windows)]
            #[allow(unused_unsafe)]
            unsafe {
                println!("[KRSDK] DLL Unloading");
                //let _ = FreeConsole();
            }
        }
        _ => {}
    }
    1
}

#[no_mangle]
pub extern "C" fn kurosdk_initSdk() {
    println!("[KRSDK] *** kurosdk_initSdk called ***");

    std::thread::spawn(|| {
        // 1. Fetch agreement.json
        println!("[KRSDK] Fetching agreement.json...");
        let agreement_data = include_str!("../../assets/agreement.json").to_string();

        *AGREEMENT_DATA.lock().unwrap() = Some(agreement_data.clone());

        // 2. Send setAnnounceConfigData LOG
        if let Some(cb) = *CALLBACK.lock().unwrap() {
            let log_msg = format!(
                "{{\"level\":1,\"msg\":\"[KUROSDK] [KR::KRConfig::setAnnounceConfigData] data: {}\\n\"}}",
                agreement_data.replace("\"", "\\\"")
            );
            let event = CString::new("LOG").unwrap();
            let data = CString::new(log_msg).unwrap();
            cb(event.as_ptr(), data.as_ptr());
        }

        std::thread::sleep(std::time::Duration::from_millis(100));

        // 3. Send languageIsAgree LOG
        if let Some(cb) = *CALLBACK.lock().unwrap() {
            let log_msg = r#"{"level":1,"msg":"[KUROSDK] [KR::KRLoginManager::showAgreementDialog] languageIsAgree:kr_sdk_true\n"}"#;
            let event = CString::new("LOG").unwrap();
            let data = CString::new(log_msg).unwrap();
            cb(event.as_ptr(), data.as_ptr());
        }

        std::thread::sleep(std::time::Duration::from_millis(100));

        // 4. Fetch SDK config
        println!("[KRSDK] Fetching SDK config...");
        let device_id = uuid::Uuid::new_v4().to_string();
        let _form_data = format!(
            "__e__=1&channelId=240&deviceNum={}&pkg=com.kurogame.pc.punishing.grayraven.en&plat=2&platform=PC&productId=A1728&productKey=2888e3853aa147ada508e38dfddb4377&projectId=G143&redirect_uri=1&version=2.6.1h",
            device_id
        );

        let config_data = include_str!("../../assets/conf.json").to_string();

        *SDK_CONFIG.lock().unwrap() = Some(config_data.clone());

        std::thread::sleep(std::time::Duration::from_millis(100));

        // 5. Send all the config LOGs
        if let Some(cb) = *CALLBACK.lock().unwrap() {
            let logs = vec![
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRHttpClient::loginInitRequest] request conf result:0\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] kefuSessionExpireHour: 1\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] client Focus value: 1\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] enableTransparentJiYanWebviewForWin value : 0\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] ignoreCheckWndVisibleForWin value: 0\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] webviewIgnoreCertErrorForWin value: 0\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] disableAgreementRemainDlgForWin value: 0\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] PayEventFixedForPCSteam value: 1\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] m_isSobotCustomerServiceEnable value: 1\n"}"#,
                r#"{"level":1,"msg":"[KUROSDK] [KR::KRConfig::setPlayerConfigData] m_clientFocusEnableForSteamDeck: 1\n"}"#,
            ];

            for log in logs {
                let event = CString::new("LOG").unwrap();
                let data = CString::new(log).unwrap();
                cb(event.as_ptr(), data.as_ptr());
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));

        // 6. NOW send PROTOCOL_AGREED and INIT_SDK_FINISHED
        if let Some(cb) = *CALLBACK.lock().unwrap() {
            println!("[KRSDK] Sending PROTOCOL_AGREED");
            let event1 = CString::new("PROTOCOL_AGREED").unwrap();
            let data1 = CString::new("").unwrap();
            cb(event1.as_ptr(), data1.as_ptr());

            std::thread::sleep(std::time::Duration::from_millis(50));

            println!("[KRSDK] Sending INIT_SDK_FINISHED");
            let event2 = CString::new("INIT_SDK_FINISHED").unwrap();
            let data2 = CString::new("").unwrap();
            cb(event2.as_ptr(), data2.as_ptr());

            SDK_INITIALIZED.store(true, Ordering::SeqCst);
            SDK_INIT_FLAG_1.store(true, Ordering::SeqCst);
            SDK_INIT_FLAG_2.store(true, Ordering::SeqCst);
            println!("[KRSDK] Init complete - login should be available");
        }
    });
}

#[no_mangle]
pub extern "C" fn kurosdk_registerSdkGlobalCallback(
    callback: extern "C" fn(*const c_char, *const c_char),
) {
    println!(
        "[KRSDK] *** kurosdk_registerSdkGlobalCallback called - callback at: {:?} ***",
        callback as *const ()
    );
    *CALLBACK.lock().unwrap() = Some(callback);
}

#[no_mangle]
pub extern "C" fn kurosdk_unregisterSdkGlobalCallback() {
    println!("[KRSDK] *** kurosdk_unregisterSdkGlobalCallback called",);
    *CALLBACK.lock().unwrap() = None;
}

#[no_mangle]
pub extern "C" fn kurosdk_initEnv() -> u8 {
    println!("[KRSDK] *** kurosdk_initEnv called ***");
    1
}

#[no_mangle]
pub extern "C" fn kr_sdk_initialize(
    a1: *const c_char,
    a2: *const c_char,
    a3: *const c_char,
    a4: *const c_char,
    a5: *const c_char,
) -> u32 {
    println!("[KRSDK] *** kr_sdk_initialize called ***");

    unsafe {
        if !a1.is_null() {
            if let Ok(s) = std::ffi::CStr::from_ptr(a1).to_str() {
                println!("[KRSDK]   param1: {}", s);
            }
        }
        if !a2.is_null() {
            if let Ok(s) = std::ffi::CStr::from_ptr(a2).to_str() {
                println!("[KRSDK]   param2: {}", s);
            }
        }
        if !a3.is_null() {
            if let Ok(s) = std::ffi::CStr::from_ptr(a3).to_str() {
                println!("[KRSDK]   param3: {}", s);
            }
        }
        if !a4.is_null() {
            if let Ok(s) = std::ffi::CStr::from_ptr(a4).to_str() {
                println!("[KRSDK]   param4: {}", s);
            }
        }
        if !a5.is_null() {
            if let Ok(s) = std::ffi::CStr::from_ptr(a5).to_str() {
                println!("[KRSDK]   param5: {}", s);
            }
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn kr_sdk_null_initialize() -> u32 {
    println!("[KRSDK] *** kr_sdk_null_initialize called ***");

    0
}

#[no_mangle]
pub extern "C" fn kurosdk_getAgreementData() -> *const c_char {
    println!("[KRSDK] *** kurosdk_getAgreementData called ***");

    if let Some(data) = AGREEMENT_DATA.lock().unwrap().as_ref() {
        let cstring = CString::new(data.as_str()).unwrap();
        let ptr = cstring.as_ptr();
        *AGREEMENT_CSTRING.lock().unwrap() = Some(cstring);
        ptr
    } else {
        std::ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn kurosdk_getSdkConfig() -> *const c_char {
    println!("[KRSDK] *** kurosdk_getSdkConfig called ***");

    if let Some(data) = SDK_CONFIG.lock().unwrap().as_ref() {
        let cstring = CString::new(data.as_str()).unwrap();
        let ptr = cstring.as_ptr();
        *CONFIG_CSTRING.lock().unwrap() = Some(cstring);
        ptr
    } else {
        std::ptr::null()
    }
}
