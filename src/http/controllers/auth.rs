use anyhow::Context;
use argon2::{Argon2, PasswordHash, password_hash::SaltString};
use axum::{Json, debug_handler, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::http::{
    ApiContext,
    error::AppError,
    extractors::AuthUser,
    repos::model::{User, UserId, UserSafe},
};

#[derive(Serialize, Deserialize)]
pub struct LogInData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogInConfirmation {
    pub id: UserId,
    pub token: String,
}

pub async fn login(
    ctx: State<ApiContext>,
    Json(data): Json<LogInData>,
) -> Result<Json<LogInConfirmation>, AppError> {
    let user: User = ctx
        .user_repo
        .get_by_username(data.username.as_str())
        .await
        .map_err(|_| AppError {
            status: StatusCode::UNAUTHORIZED,
            message: "Invalid credentials".to_string(),
        })?;

    verify_password(data.password, user.password)
        .await
        .map_err(|_| AppError {
            status: StatusCode::UNAUTHORIZED,
            message: "Invalid credentials".to_string(),
        })?;

    Ok(Json(LogInConfirmation {
        id: user.id,
        token: AuthUser { user_id: user.id }.to_jwt(&ctx),
    }))
}

#[derive(Serialize, Deserialize)]
pub struct SignUpData {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[debug_handler]
pub async fn signup(
    ctx: State<ApiContext>,
    Json(data): Json<SignUpData>,
) -> Result<Json<UserSafe>, AppError> {
    let password_hash = hash_password(data.password).await?;
    let new_user = User {
        id: uuid::Uuid::new_v4(),
        username: data.username,
        password: password_hash,
        email: data.email,
    };

    let user = ctx.user_repo.create(new_user).await?;

    Ok(Json(user))
}

pub async fn hash_password(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || -> Result<String, AppError> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_salt())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("error generating password hash")?
}

pub async fn verify_password(password: String, hash: String) -> Result<(), AppError> {
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let hash = PasswordHash::new(&hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => AppError {
                    status: StatusCode::UNAUTHORIZED,
                    message: "Invalid password".into(),
                },
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("error verifying password hash")?
}
