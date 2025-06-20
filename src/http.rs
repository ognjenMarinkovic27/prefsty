use crate::http::repos::{game::GameRepo, user::UserRepo};
use crate::http::ws::ClientMap;
use std::sync::Arc;

pub mod controllers;
pub mod error;
pub mod extractors;
pub mod repos;
pub mod routes;
pub mod ws;

#[derive(Debug, Clone)]
pub(super) struct ApiContext {
    pub(super) config: ApiConfig,
    pub(super) game_repo: Arc<GameRepo>,
    pub(super) user_repo: Arc<UserRepo>,
    pub(super) clients: Arc<ClientMap>,
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub hmac_key: String,
}
