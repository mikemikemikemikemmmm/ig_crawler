use crate::{_struct, fetch};
use axum::{Json, extract::Path, response::IntoResponse};
use std::fmt;
#[derive(Debug)]
pub struct HandlerError {
    detail: String,
}

impl HandlerError {
    fn new(msg: impl Into<String>) -> Self {
        Self { detail: msg.into() }
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detail)
    }
}
impl IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        // 轉成 HTTP 回應
        (axum::http::StatusCode::BAD_REQUEST, self.detail).into_response()
    }
}

pub async fn get_ig_detail(
    Path(user_name): Path<String>,
) -> Result<Json<_struct::count::User>, HandlerError> {
    if user_name.len() > 40 {
        return Err(HandlerError::new("username is too long"));
    }
    let user_pk = fetch::fetch_ig_to_get_user_pk(&user_name)
        .await
        .map_err(|e| HandlerError::new(e))?;
    let user_data = fetch::fetch_ig_to_get_user_data(user_pk)
        .await
        .map_err(|e| {
            HandlerError::new(e)
        })?;
    Ok(Json(user_data))
}
