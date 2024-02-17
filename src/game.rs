use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameSources {
    Tournament,
    League,
    Arena,
    Challenge,
    Custom,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruleset {
    name: String,
    version: String,
    settings: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub releset: Ruleset,
    pub map: String,
    pub timeout: String,
    pub source: GameSources,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Vec2>,
    hazards: Vec<Vec2>,
    snakes: Vec<Battlesnake>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customizations {
    color: String,
    head: String,
    tail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: i32,
    body: Vec<Vec2>,
    latency: String,
    head: Vec2,
    lenght: i32,
    shout: String,
    squard: String,
    customizations: Customizations,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineInput {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Battlesnake,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
enum Movements {
    Up,
    Down,
    Left,
    Right,
}

impl Movements {
    const ALL: [Movements; 4] = [
        Movements::Up,
        Movements::Down,
        Movements::Left,
        Movements::Right,
    ];

    fn coords(&self) -> Vec2 {
        match self {
            Movements::Up => Vec2::new(0.0, 1.0),
            Movements::Down => Vec2::new(0.0, -1.0),
            Movements::Left => Vec2::new(-1.0, 0.0),
            Movements::Right => Vec2::new(1.0, 0.0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoveResponse {
    pub r#move: Movements,
    pub shout: Option<String>,
}

impl MoveResponse {
    const _SHOUT_MAX_LENGTH: usize = 256;
}

/// Handles POST /start ... used to start a new game
/// response is irrelevant as the battlesnake engine does not care
pub async fn handle_game_start(
    Json(_input): Json<EngineInput>,
) -> impl IntoResponse {
    //TODO: process the game start metadata
    tracing::info!("Game started");
    StatusCode::OK
}

/// Handles POST /end ... used to end the current game
/// response is irrelevant as the battlesnake engine does not care
pub async fn handle_game_over(
    Json(_input): Json<EngineInput>,
) -> impl IntoResponse {
    //TODO: log the game over event
    tracing::info!("Game started");
    StatusCode::OK
}

/// Handles POST /move ... used to move the snake
pub async fn handle_move(Json(input): Json<EngineInput>) -> impl IntoResponse {
    let head = &input.you.head;
    let body = &input.you.body;
    let obstacles = &input.board.hazards;
    let food = &input.board.food;
    let height = input.board.height;
    let width = input.board.width;

    let possible_moves = Movements::ALL
        .iter()
        .filter(|m| {
            let next = *head + m.coords();
            let inside_map = next.x >= 0.0
                && next.y >= 0.0
                && next.x < width as f32
                && next.y < height as f32;
            let colision = body.contains(&next) || obstacles.contains(&next);

            inside_map && !colision
        })
        .collect::<Vec<_>>();

    let closest_food = food
        .iter()
        .min_by_key(|f| {
            let dist = (f.x - head.x).abs() + (f.y - head.y).abs();
            dist as i32
        })
        .unwrap();

    let closest_move_to_food = possible_moves
        .iter()
        .min_by_key(|m| {
            let next = *head + m.coords();
            let dist = (closest_food.x - next.x).abs()
                + (closest_food.y - next.y).abs();
            dist as i32
        })
        .unwrap();

    let response = MoveResponse {
        r#move: **closest_move_to_food,
        shout: None,
    };

    tracing::info!("Move: {:?}", response.r#move);
    (StatusCode::OK, Json(response))
}
