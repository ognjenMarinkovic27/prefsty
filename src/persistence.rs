use sqlx::{
    Decode, Encode, Postgres, Type,
    postgres::{PgTypeInfo, PgValueRef},
};

use crate::core::game::GameState;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GameModel {
    id: uuid::Uuid,
    state: GameState,
}

impl<'r> Decode<'r, Postgres> for GameState {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let json: serde_json::Value = Decode::<'r, Postgres>::decode(value)?;
        Ok(serde_json::from_value(json)?)
    }
}

impl<'q> Encode<'q, Postgres> for GameState {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let json = serde_json::to_value(self).unwrap();
        json.encode_by_ref(buf)
    }

    fn size_hint(&self) -> usize {
        let json = serde_json::to_value(self).unwrap();
        json.size_hint()
    }
}

impl Type<Postgres> for GameState {
    fn type_info() -> PgTypeInfo {
        <serde_json::Value as Type<Postgres>>::type_info()
    }
}
pub struct PgDB {
    pool: sqlx::PgPool,
}

impl PgDB {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn load_game(&self, id: uuid::Uuid) -> std::result::Result<GameModel, sqlx::Error> {
        let rec = sqlx::query_as!(
            GameModel,
            "SELECT id, state as \"state: _\" FROM games WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(rec)
    }

    pub async fn save_game(
        &self,
        game: GameModel,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            r#"
            INSERT INTO games (id, state)
            VALUES ($1, $2)
            ON CONFLICT (id) DO UPDATE
            SET state = EXCLUDED.state
            "#,
            game.id,
            serde_json::to_value(&game.state)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
