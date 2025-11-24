use crate::types::*;

const SERVER_URL: &str = "http://127.0.0.1";

pub fn login(username: &str, password: &str) -> Result<UserSession, String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{}/api/AscNet/login", SERVER_URL))
        .json(&LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        })
        .send()
        .map_err(|e| format!("Network error: {}", e))?;

    // Debug: Print the raw response
    let response_text = response
        .text()
        .map_err(|e| format!("Failed to read response: {}", e))?;
    println!("[Login] Server response: {}", response_text);

    // Try to parse it
    let data: LoginResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Parse error: {} | Response was: {}", e, response_text))?;

    if data.code == 0 {
        if let Some(account) = data.account {
            Ok(UserSession {
                username: account.username,
                token: account.token,
                uid: account.uid.to_string(),
            })
        } else {
            Err("No account data returned".to_string())
        }
    } else {
        Err(data.msg)
    }
}

pub fn register(username: &str, password: &str) -> Result<UserSession, String> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post(format!("{}/api/AscNet/register", SERVER_URL))
        .json(&LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        })
        .send()
        .map_err(|e| format!("Network error: {}", e))?;

    let data: LoginResponse = response.json().map_err(|e| format!("Parse error: {}", e))?;

    if data.code == 0 {
        if let Some(account) = data.account {
            Ok(UserSession {
                username: account.username,
                token: account.token,
                uid: account.uid.to_string(),
            })
        } else {
            Err("No account data returned".to_string())
        }
    } else {
        Err(data.msg)
    }
}
