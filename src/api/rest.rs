use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub data: serde_json::Value,
}

pub fn rest_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health_check = warp::path("health")
        .and(warp::get())
        .map(|| {
            let response = ApiResponse {
                status: "ok".to_string(),
                data: serde_json::json!({}),
            };
            warp::reply::json(&response)
        });

    health_check
}   