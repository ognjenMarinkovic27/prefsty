use crate::http::repos::{
    error::DbError,
    model::{GameModel, UserSafe},
};

#[derive(Debug)]
pub struct GameRepo {
    pool: sqlx::PgPool,
}

impl GameRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<GameModel>, DbError> {
        let rec = sqlx::query_as!(
            GameModel,
            "SELECT id, state as \"state: _\", created_by FROM games",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => DbError::NotFound(e),
            _ => e.into(),
        })?;

        Ok(rec)
    }

    pub async fn get_by_id(&self, id: uuid::Uuid) -> Result<GameModel, DbError> {
        let rec = sqlx::query_as!(
            GameModel,
            "SELECT id, state as \"state: _\", created_by FROM games WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => DbError::NotFound(e),
            _ => e.into(),
        })?;

        Ok(rec)
    }

    pub async fn update(&self, game: GameModel) -> anyhow::Result<(), DbError> {
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

    pub async fn create(&self, game: GameModel) -> anyhow::Result<(), DbError> {
        sqlx::query!(
            r#"
            INSERT INTO games (id, state, created_by)
            VALUES ($1, $2, $3)
            "#,
            game.id,
            serde_json::to_value(&game.state)?,
            game.created_by
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn join(&self, game_id: uuid::Uuid, user_id: uuid::Uuid) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            INSERT INTO joined (game_id, user_id)
            VALUES ($1, $2)
            "#,
            game_id,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_joined_by_game_id(
        &self,
        game_id: uuid::Uuid,
    ) -> Result<Vec<UserSafe>, DbError> {
        let rec = sqlx::query_as!(
            UserSafe,
            "SELECT users.id, users.username
            FROM users
            JOIN joined ON users.id = joined.user_id
            WHERE joined.game_id = $1",
            game_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rec)
    }
}
