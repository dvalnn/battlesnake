#![allow(dead_code)]

mod config;
mod game;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(config::get_config))
        .route("/start", get(game::handle_game_start))
        .route("/end", get(game::handle_game_over))
        .route("/move", get(game::handle_move));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
