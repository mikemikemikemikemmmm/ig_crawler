use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PkResponse {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "xdt_api__v1__feed__user_timeline_graphql_connection")]
    pub feed: FeedConnection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedConnection {
    pub edges: Vec<Edge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub node: Node,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
}