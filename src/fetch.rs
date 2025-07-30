use crate::_struct;
use crate::config;
use crate::csrf_token;
use crate::utils::{
    get_user_count_by_response, get_user_follower_count_value_payload, get_user_pk_by_response,
    get_user_pk_value_payload,
};
use anyhow::Result;
use reqwest;
use reqwest::Client;
use reqwest::header::USER_AGENT;
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
    let user_data = get_user_count_by_response(response).await.map_err(|e| e)?;
    Ok(user_data)
}

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
    let user_pk = get_user_pk_by_response(res).await.map_err(|e| e)?;
    Ok(user_pk)
}
