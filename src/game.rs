use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

/// Handles POST /start ... used to start a new game
/// response is irrelevant as the battlesnake engine does not care
pub async fn handle_game_start() -> impl IntoResponse {
    StatusCode::OK
}

/// Handles POST /end ... used to end the current game
/// response is irrelevant as the battlesnake engine does not care
pub async fn handle_game_over() -> impl IntoResponse {
    StatusCode::OK
}

/// Handles POST /move ... used to move the snake
pub async fn handle_move() -> impl IntoResponse {
    let response = MoveResponse {
        r#move: Movements::Up,
        shout: None,
    };
    (StatusCode::OK, Json(response))
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Movements {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoveResponse {
    pub r#move: Movements,
    pub shout: Option<String>,
}

impl MoveResponse {
    const SHOUT_MAX_LENGTH: usize = 256;
}
