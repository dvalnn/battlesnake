use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct SnakeConfig {
    pub apiversion: &'static str,
    pub author: &'static str,
    pub color: &'static str,
    pub head: &'static str,
    pub tail: &'static str,
    pub version: &'static str,
}

impl SnakeConfig {
    const API_VERSION: &'static str = "1";
    const AUTHOR: &'static str = "dvalinn";
    const COLOR: &'static str = "#ff00ff";
    const HEAD: &'static str = "default";
    const TAIL: &'static str = "default";
    const VERSION: &'static str = "0.0.1-alpha.0";

    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SnakeConfig {
    fn default() -> Self {
        SnakeConfig {
            apiversion: Self::API_VERSION,
            author: Self::AUTHOR,
            color: Self::COLOR,
            head: Self::HEAD,
            tail: Self::TAIL,
            version: Self::VERSION,
        }
    }
}

pub async fn get_config() -> impl IntoResponse {
    (
        axum::http::StatusCode::OK,
        axum::Json(SnakeConfig::default()),
    )
}
