use crate::_struct;
use anyhow::Result;
use reqwest;

pub fn get_user_pk_value_payload(username: &str) -> String {
    format!(
        r#"{{"data":{{"count": 1}},"username": "{}","__relay_internal__pv__PolarisIsLoggedInrelayprovider": false,"__relay_internal__pv__PolarisShareSheetV3relayprovider": false}}"#,
        username
    )
}
pub fn get_user_follower_count_value_payload(user_pk: &str) -> String {
    format!(r#"{{"id":{},"render_surface":"PROFILE"}}"#, user_pk)
}
pub async fn get_user_pk_by_response(res: reqwest::Response) -> Result<String, &'static str> {
    let body_text = res
        .text()
        .await
        .map_err(|_| "Failed to read response text")?;

    print!("{:?}", body_text);
    let result_json: _struct::pk::GetPkResponse =
        serde_json::from_str(&body_text).map_err(|_| "Failed to parse JSON")?;
    let target_str = result_json.data.user_timeline.page_info.end_cursor;

    target_str
        .split('_')
        .nth(1)
        .map(|v| v.to_string())
        .ok_or("Failed to find user pk in end_cursor")
}
pub async fn get_user_count_by_response(
    res: reqwest::Response,
) -> Result<_struct::count::User, &'static str> {
    let body_text = res
        .text()
        .await
        .map_err(|_| "Failed to read response text")?;
    let result_json: _struct::count::CountResponse =
        serde_json::from_str(&body_text).map_err(|_| "Failed to parse JSON")?;
    Ok(result_json.data.user)
}
