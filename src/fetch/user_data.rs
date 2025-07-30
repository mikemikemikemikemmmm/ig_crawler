use crate::_struct;
use crate::config;
use crate::csrf_token;
use anyhow::Result;
use reqwest;
use reqwest::Client;
use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use std::collections::HashMap;
pub async fn fetch_ig_to_get_user_data(user_pk: String) -> Result<_struct::count::User, String> {
    let csrf_token = csrf_token::GLOBAL_CSRF_TOKEN.read().await.to_string();
    let client = Client::new();
    let mut form_data = HashMap::new();
    form_data.insert("doc_id", config::GET_DATA_DOC_ID);
    let payload = get_user_follower_count_value_payload(&user_pk);
    form_data.insert("variables", &payload);
    let response = client
        .post(config::IG_QUERY_URL)
        .form(&form_data)
        .header("X-CSRFToken", csrf_token)
        .header("X-IG-App-ID", config::GET_X_IG_APP_ID)
        .header(USER_AGENT, config::USER_AGENT)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if response.status() != StatusCode::OK {
        return Err("err when fetch user data.".to_string());
    }
    let user_data = get_user_data_by_response(response).await.map_err(|e| e)?;
    Ok(user_data)
}


pub async fn get_user_data_by_response(
    res: reqwest::Response,
) -> Result<_struct::count::User, String> {
    let body_text = res
        .text()
        .await
        .map_err(|e|e.to_string())?;
    print!("{:?}",body_text);
    let result_json: _struct::count::CountResponse =
        serde_json::from_str(&body_text).map_err(|e| e.to_string())?;
    Ok(result_json.data.user)
}

pub fn get_user_follower_count_value_payload(user_pk: &str) -> String {
    format!(r#"{{"id":{},"render_surface":"PROFILE"}}"#, user_pk)
}