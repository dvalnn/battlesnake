use std::collections::HashMap;

use axum::{http::StatusCode, response::IntoResponse, Json};
use glam::IVec2;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruleset {
    name: String,
    version: String,
    settings: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub ruleset: Ruleset,
    pub map: String,
    pub timeout: i32,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl From<Coord> for IVec2 {
    fn from(coord: Coord) -> Self {
        IVec2::new(coord.x, coord.y)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    height: i32,
    width: i32,
    food: Vec<Coord>,
    hazards: Vec<Coord>,
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
    body: Vec<Coord>,
    latency: String,
    head: Coord,
    length: i32,
    shout: String,
    squad: Option<String>,
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
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

impl Movements {
    const ALL: [Movements; 4] = [
        Movements::Up,
        Movements::Down,
        Movements::Left,
        Movements::Right,
    ];

    fn coords(&self) -> IVec2 {
        match self {
            Movements::Up => IVec2::new(0, 1),
            Movements::Down => IVec2::new(0, -1),
            Movements::Left => IVec2::new(-1, 0),
            Movements::Right => IVec2::new(1, 0),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoveResponse {
    pub r#move: Movements,
    pub shout: String,
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
    tracing::info!("Game ended");
    StatusCode::OK
}

/// Handles POST /move ... used to move the snake
pub async fn handle_move(Json(input): Json<EngineInput>) -> impl IntoResponse {
    tracing::debug!("Last move latency: {}", input.you.latency);

    let head = IVec2::from(input.you.head);

    let body = input
        .you
        .body
        .into_iter()
        .map(IVec2::from)
        .collect::<Vec<_>>();

    let obstacles = input
        .board
        .hazards
        .into_iter()
        .map(IVec2::from)
        .collect::<Vec<_>>();

    let food = input
        .board
        .food
        .into_iter()
        .map(IVec2::from)
        .collect::<Vec<_>>();

    let height = input.board.height;
    let width = input.board.width;

    let possible_moves = Movements::ALL
        .iter()
        .filter(|m| {
            let next = head + m.coords();
            let inside_map =
                next.x >= 0 && next.y >= 0 && next.x < width && next.y < height;
            let colision = body.contains(&next) || obstacles.contains(&next);

            inside_map && !colision
        })
        .collect::<Vec<_>>();

    let closest_food = food
        .iter()
        .min_by_key(|f| (f.x - head.x).abs() + (f.y - head.y).abs())
        .expect("There is not food!");

    let mut shout = format!(
        "I'm at {:?} and the closest food is at {:?}",
        head, closest_food
    );

    let closest_move_to_food = possible_moves
        .iter()
        .min_by_key(|m| {
            let next = head + m.coords();
            (closest_food.x - next.x).abs() + (closest_food.y - next.y).abs()
        })
        .unwrap_or_else(|| {
            shout = "Im doomed!".to_string();
            &&Movements::Up
        });

    let response = MoveResponse {
        r#move: **closest_move_to_food,
        shout,
    };

    tracing::info!("Turn: {:?} | Move: {:?}", input.turn, response.r#move);
    (StatusCode::OK, Json(response))
}
