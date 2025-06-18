use prefsty::core::game::GameState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameModel {
    pub id: uuid::Uuid,
    pub state: GameState,
    pub created_by: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSafe {
    pub id: uuid::Uuid,
    pub username: String,
}
