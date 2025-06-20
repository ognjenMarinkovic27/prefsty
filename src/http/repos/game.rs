use crate::http::repos::{
    error::DbError,
    model::{Game, GameId, UserId, UserSafe, UserSafeIdx},
};

#[derive(Debug)]
pub struct GameRepo {
    pool: sqlx::PgPool,
}

impl GameRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Game>, DbError> {
        let rec = sqlx::query_as!(
            Game,
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

    pub async fn get_by_id(&self, id: GameId) -> Result<Game, DbError> {
        let rec = sqlx::query_as!(
            Game,
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

    pub async fn update(&self, game: &Game) -> anyhow::Result<(), DbError> {
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

    pub async fn create(&self, game: Game) -> anyhow::Result<(), DbError> {
        const PLAYER_COUNT: i32 = 3;
        let mut tx = self.pool.begin().await?;
        sqlx::query!(
            r#"
            INSERT INTO games (id, state, created_by)
            VALUES ($1, $2, $3)
            "#,
            game.id,
            serde_json::to_value(&game.state)?,
            game.created_by
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
            INSERT INTO joined (game_id, idx)
            SELECT $1, generate_series(0, $2::int - 1)
            "#,
            game.id,
            PLAYER_COUNT
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn join(&self, game_id: GameId, user_id: UserId) -> Result<(), DbError> {
        let opt_idx: Option<i16> = sqlx::query_scalar!(
            r#"
            WITH sel AS (
                SELECT   idx
                FROM     joined
                WHERE    game_id  = $1
                AND      user_id IS NULL
                ORDER BY idx
                LIMIT    1
            )
            UPDATE    joined
            SET       user_id = $2
            FROM      sel
            WHERE     joined.game_id = $1
            AND       joined.idx     = sel.idx
            RETURNING joined.idx;
            "#,
            game_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        opt_idx.ok_or(DbError::NoAvailableSlot)?;

        Ok(())
    }

    pub async fn get_joined_by_game_id(
        &self,
        game_id: GameId,
    ) -> Result<Vec<UserSafeIdx>, DbError> {
        let rec = sqlx::query_as!(
            UserSafeIdx,
            "SELECT users.id, users.username, joined.idx
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
