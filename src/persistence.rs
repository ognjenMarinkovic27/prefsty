use crate::persistence::{error::PersistenceError, model::GameModel};

pub mod error;
pub mod model;
pub struct PgDB {
    pool: sqlx::PgPool,
}

impl PgDB {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn load_game(
        &self,
        id: uuid::Uuid,
    ) -> std::result::Result<GameModel, PersistenceError> {
        let rec = sqlx::query_as!(
            GameModel,
            "SELECT id, state as \"state: _\" FROM games WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn update_game(&self, game: GameModel) -> Result<(), PersistenceError> {
        sqlx::query!(
            r#"
            UPDATE games
            SET state = $2
            WHERE id = $1
            "#,
            game.id,
            serde_json::to_value(&game.state)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_game(&self, game: GameModel) -> Result<(), PersistenceError> {
        sqlx::query!(
            r#"
            INSERT INTO games (id, state)
            VALUES ($1, $2)
            "#,
            game.id,
            serde_json::to_value(&game.state)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
