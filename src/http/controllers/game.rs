use anyhow::Result;
use axum::{
    Json,
    extract::{Path, State},
};
use prefsty::core::game::new_game;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http::{
    ApiContext,
    error::AppError,
    extractors::AuthUser,
    repos::model::{GameId, GameModel, UserSafe},
};

#[axum::debug_handler]
pub async fn get_all(
    _: AuthUser,
    ctx: State<ApiContext>,
) -> Result<Json<Vec<GameModel>>, AppError> {
    let games_repo = &ctx.game_repo;
    let games = games_repo.get_all().await?;

    Ok(Json(games))
}

pub async fn get_by_id(
    _: AuthUser,
    ctx: State<ApiContext>,
    Path(game_id): Path<GameId>,
) -> Result<Json<GameModel>, AppError> {
    let games_repo = &ctx.game_repo;
    let game = games_repo.get_by_id(game_id).await?;

    Ok(Json(game))
}

pub async fn get_joined_by_game_id(
    _: AuthUser,
    ctx: State<ApiContext>,
    Path(game_id): Path<GameId>,
) -> Result<Json<Vec<UserSafe>>, AppError> {
    let games_repo = &ctx.game_repo;
    let joined_users = games_repo.get_joined_by_game_id(game_id).await?;

    Ok(Json(joined_users))
}

pub async fn join(
    user: AuthUser,
    ctx: State<ApiContext>,
    Path(game_id): Path<GameId>,
) -> Result<Json<()>, AppError> {
    let games_repo = &ctx.game_repo;
    games_repo.join(game_id, user.user_id).await?;

    Ok(Json(()))
}

#[derive(Serialize, Deserialize)]
pub struct NewGameSettings {
    first: usize,
    starting_score: u32,
    num_refas: usize,
}

#[axum::debug_handler]
pub async fn create(
    user: AuthUser,
    ctx: State<ApiContext>,
    Json(settings): Json<NewGameSettings>,
) -> Result<Json<()>, AppError> {
    let new_game = new_game(settings.first, settings.starting_score, settings.num_refas);

    let games_repo = &ctx.game_repo;
    games_repo
        .create(GameModel {
            id: Uuid::new_v4(),
            state: new_game,
            created_by: user.user_id,
        })
        .await?;

    Ok(Json(()))
}
