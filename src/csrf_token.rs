use crate::config::{self};
use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use reqwest;
use reqwest::Client;
use reqwest::header::USER_AGENT;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::sync::watch;
static CSRF_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"csrftoken=([^;]+)").unwrap());

pub static GLOBAL_CSRF_TOKEN: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

async fn fetch_csrf_token() -> Result<String, String> {
    let client = Client::new();
    let response = client
        .get(config::IG_URL)
        .header(USER_AGENT, config::USER_AGENT)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let cookie_value = response
        .headers()
        .get("set-cookie")
        .ok_or("get cookie error")?;
    let cookie_str = cookie_value
        .to_str()
        .map_err(|e| e.to_string())?;
    let csrf_token = CSRF_REGEX.captures(cookie_str).ok_or("regex csrf err")?;

    Ok(csrf_token[1].to_owned())
}

pub async fn update_csrf_token(shutdown_tx: watch::Sender<bool>) {
    loop {
        match fetch_csrf_token().await {
            Ok(new_csrf_token) => {
                let mut write_guard = GLOBAL_CSRF_TOKEN.write().await;
                let cloned = new_csrf_token.clone();
                *write_guard = new_csrf_token; // 直接改寫鎖內的 String
                println!("global csrf token updated!, new csrf token={}", cloned);
            }
            Err(e) => {
                eprintln!("update_csrf_token error: {}", e);
                let _ = shutdown_tx.send(true);
            }
        };

        tokio::time::sleep(Duration::from_secs(config::RENEW_CSRF_TOKEN_MIN*60)).await; // 30分鐘
    }
}
