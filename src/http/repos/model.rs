use prefsty::core::game::GameState;
use serde::{Deserialize, Serialize};

pub type UserId = uuid::Uuid;
pub type GameId = uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: GameId,
    pub state: GameState,
    pub created_by: UserId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSafe {
    pub id: UserId,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSafeIdx {
    pub id: UserId,
    pub username: String,
    pub idx: i16,
}
