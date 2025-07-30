use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GetPkResponse {
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "xdt_api__v1__feed__user_timeline_graphql_connection")]
   pub user_timeline: UserTimelineConnection,
}

#[derive(Deserialize, Debug)]
pub struct UserTimelineConnection {
   pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
pub struct PageInfo {
   pub end_cursor: String,
}

// 