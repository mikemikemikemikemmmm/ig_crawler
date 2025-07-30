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
    println!("req user name : {}", &user_name);
    if user_name.len() > 40 {
        return Err(HandlerError::new("username is too long"));
    }
    let user_pk = fetch::pk::fetch_ig_to_get_user_pk(&user_name)
        .await
        .map_err(|e| {
            println!("fetch user pk failed.");
            HandlerError::new(e)
        })?;
    println!("req user pk : {}", &user_pk);
    let user_data = fetch::user_data::fetch_ig_to_get_user_data(user_pk)
        .await
        .map_err(|e| {
            println!("fetch user data failed.");
            HandlerError::new(e)
        })?;
    println!("req user media count : {}", &user_data.media_count);
    println!("req user follower_count : {}", &user_data.follower_count);
    println!("req user following_count : {}", &user_data.following_count);
    // let user_data: _struct::count::User = _struct::count::User {
    //     follower_count: 1,
    //     following_count: 2,
    //     media_count: 2,
    // };
    Ok(Json(user_data))
}
