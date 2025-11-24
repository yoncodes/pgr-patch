use crate::globals::{SendPtr, ALLOCATED_STRINGS, LANGUAGE};
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn kurosdk_getConfigInfo() -> *mut c_char {
    println!("[KRSDK] *** kurosdk_getConfigInfo called ***");

    let config = serde_json::json!({
        "channelId": "240",
        "channelName": "PC",
        "channelOp": "null",
        "gameId": "G143",
        "pkgId": "A1728"
    });

    let json_str = config.to_string();
    println!("[KRSDK] Returning config: {}", json_str);

    let c_str = CString::new(json_str).unwrap();
    let ptr = c_str.into_raw();

    ALLOCATED_STRINGS.lock().unwrap().push(SendPtr(ptr));
    ptr
}

#[no_mangle]
pub extern "C" fn kurosdk_getDeviceInfo() -> *mut c_char {
    println!("[KRSDK] *** kurosdk_getDeviceInfo called ***");

    let device_info = serde_json::json!({
        "did": uuid::Uuid::new_v4().to_string(),
        "idfv": "",
        "jyDid": "",
        "oaId": ""
    });

    let json_str = device_info.to_string();
    println!("[KRSDK] Returning device info: {}", json_str);

    let c_str = CString::new(json_str).unwrap();
    let ptr = c_str.into_raw();

    // Keep track of allocated strings
    ALLOCATED_STRINGS.lock().unwrap().push(SendPtr(ptr));

    ptr
}

#[no_mangle]
pub extern "C" fn kurosdk_getProtocolInfo() -> *mut c_char {
    println!("[KRSDK] *** Get Protocol Info called ***");

    let protocol_info = serde_json::json!({"data": []});

    let json_str = protocol_info.to_string();
    println!("[KRSDK] Returning protocol info: {}", json_str);

    let c_ctr = CString::new(json_str).unwrap();
    let ptr = c_ctr.into_raw();

    ALLOCATED_STRINGS.lock().unwrap().push(SendPtr(ptr));
    ptr
}

#[no_mangle]
pub extern "C" fn kurosdk_setLanguage(data: *const c_char) {
    println!("[KRSDK] *** kurosdk_setLanguage called ***");

    if data.is_null() {
        println!("[KRSDK] setLanguage: null data");
        return;
    }

    unsafe {
        let c_str = std::ffi::CStr::from_ptr(data);
        if let Ok(json_str) = c_str.to_str() {
            println!("[KRSDK] setLanguage data: {}", json_str);

            // Parse JSON to get language
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(lang) = json.get("language").and_then(|v| v.as_str()) {
                    *LANGUAGE.lock().unwrap() = lang.to_string();
                    println!("[KRSDK] Language set to: {}", lang);
                }
            }
        }
    }
}
