use crate::_struct;
use crate::config;
use crate::csrf_token;
use anyhow::Result;
use reqwest;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::header::USER_AGENT;
use std::collections::HashMap;

pub async fn fetch_ig_to_get_user_pk(username: &str) -> Result<String, String> {
    let csrf_token = csrf_token::GLOBAL_CSRF_TOKEN.read().await.to_string();
    let mut form_data = HashMap::new();
    form_data.insert("doc_id", config::GET_PK_DOC_ID);
    let payload = get_user_pk_value_payload(&username);
    form_data.insert("variables", &payload);
    let client = Client::new();
    let res = client
        .post(config::IG_QUERY_URL)
        .form(&form_data)
        .header("X-CSRFToken", csrf_token)
        .header("X-IG-App-ID", config::GET_X_IG_APP_ID)
        .header(USER_AGENT, config::USER_AGENT)
        .send()
        .await
        .map_err(|_| "get pk err")?;
    let status = res.status();
    let text = res.text().await.map_err(|e| e.to_string())?;
    if status == StatusCode::UNAUTHORIZED {
        return Err("Please wait a few minutes before you try again.".to_string());
    } else if status != StatusCode::OK {
        return Err("this user not exist.".to_string());
    }
    let user_pk = get_user_pk_by_response(text).await.map_err(|_|"user not exist.".to_string())?;
    Ok(user_pk)
}

pub async fn get_user_pk_by_response(body_text: String) -> Result<String, String> {
    let result_json: _struct::pk::PkResponse =
        serde_json::from_str(&body_text).map_err(|e| e.to_string())?;
    let target_str = &result_json.data.feed.edges[0].node.id;
    target_str
        .split('_')
        .nth(1)
        .map(|v| v.to_string())
        .ok_or("Failed to find user pk in end_cursor".to_string())
}
pub fn get_user_pk_value_payload(username: &str) -> String {
    format!(
        r#"{{"data":{{"count": 1}},"username": "{}","__relay_internal__pv__PolarisIsLoggedInrelayprovider": false,"__relay_internal__pv__PolarisShareSheetV3relayprovider": false}}"#,
        username
    )
}
