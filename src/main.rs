use axum;
use dashmap::DashMap;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
mod http;

use http::ApiConfig;

use crate::http::repos::{game::GameRepo, user::UserRepo};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = TcpListener::bind("localhost:8080")
        .await
        .expect("Unable to connect to the server");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/pref")
        .await?;

    let context = http::ApiContext {
        config: ApiConfig {
            hmac_key: "bigsecretwhooo".into(),
        },
        game_repo: Arc::new(GameRepo::new(pool.clone())),
        user_repo: Arc::new(UserRepo::new(pool.clone())),
        clients: Arc::new(DashMap::new()),
    };
    let app = http::routes::app(context).await;

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .expect("Error serving application");

    Ok(())
}
