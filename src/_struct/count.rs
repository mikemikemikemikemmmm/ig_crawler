use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CountResponse {
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub user: User,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct User {
    pub follower_count: u64,
    pub following_count: u64,
    pub media_count: u64,
}
