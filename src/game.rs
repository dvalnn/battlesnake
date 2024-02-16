use axum::{http::StatusCode, response::IntoResponse, Json};
use glam::Vec2;
use serde::{Deserialize, Serialize};

/// Handles POST /start ... used to start a new game
/// response is irrelevant as the battlesnake engine does not care
pub async fn handle_game_start(
    Json(_input): Json<EngineInput>,
) -> impl IntoResponse {
    StatusCode::OK
}

/// Handles POST /end ... used to end the current game
/// response is irrelevant as the battlesnake engine does not care
pub async fn handle_game_over(
    Json(_input): Json<EngineInput>,
) -> impl IntoResponse {
    StatusCode::OK
}

/// Handles POST /move ... used to move the snake
pub async fn handle_move(Json(_input): Json<EngineInput>) -> impl IntoResponse {
    let response = MoveResponse {
        r#move: Movements::Up,
        shout: None,
    };
    (StatusCode::OK, Json(response))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Royale {
    #[serde(alias = "shrinkEveryNTurns")]
    shrink_every_n_turns: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Squad {
    #[serde(alias = "allowBodyCollisions")]
    allow_body_collisions: bool,

    #[serde(alias = "sharedElimination")]
    shared_elimination: bool,

    #[serde(alias = "sharedHealth")]
    shared_health: bool,

    #[serde(alias = "sharedLength")]
    shared_lenght: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RulesetSettings {
    #[serde(alias = "foodSpawnChance")]
    food_spawn_chance: i32,

    #[serde(alias = "minimumFood")]
    minimum_food: i32,

    #[serde(alias = "hazardDamagePerTurn")]
    hazard_damage_per_turn: i32,
}

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
    settings: RulesetSettings,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoveResponse {
    pub r#move: Movements,
    pub shout: Option<String>,
}

impl MoveResponse {
    const SHOUT_MAX_LENGTH: usize = 256;
}
