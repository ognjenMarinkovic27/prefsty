use crate::http::{ApiContext, error::AppError, repos::model::UserId};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{HeaderValue, StatusCode, header::AUTHORIZATION, request::Parts},
};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use time::OffsetDateTime;

pub struct AuthUser {
    pub user_id: UserId,
}

#[derive(Serialize, Deserialize)]
pub struct AuthUserClaims {
    id: UserId,
    expires_at: i64,
}

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(1);

impl AuthUser {
    pub fn to_jwt(&self, ctx: &ApiContext) -> String {
        let hmac: Hmac<Sha384> =
            Hmac::new_from_slice(ctx.config.hmac_key.as_bytes()).expect("Length can't be invalid");

        AuthUserClaims {
            id: self.user_id,
            expires_at: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing should be infallible")
    }

    fn from_auth_header(value: &HeaderValue, ctx: ApiContext) -> Result<Self, AppError> {
        let token_str = value.to_str().map_err(|_| AppError {
            status: StatusCode::UNAUTHORIZED,
            message: "`Authorization` header contains unexpected characters".into(),
        })?;

        let token = token_str.strip_prefix("Bearer ").ok_or(AppError {
            status: StatusCode::UNAUTHORIZED,
            message: "`Authorization` header has invalid format. Format should be `Bearer {TOKEN}`"
                .into(),
        })?;

        Self::from_token(token, ctx)
    }

    fn from_token(token_str: &str, ctx: ApiContext) -> Result<AuthUser, AppError> {
        let hmac: Hmac<Sha384> =
            Hmac::new_from_slice(ctx.config.hmac_key.as_bytes()).map_err(|_| AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Couldn't verify signature".into(),
            })?;

        let token: jwt::Token<jwt::Header, AuthUserClaims, _> =
            token_str.verify_with_key(&hmac).map_err(|_| AppError {
                status: StatusCode::UNAUTHORIZED,
                message: "Signature verification failed".into(),
            })?;

        let claims = token.claims();

        if claims.expires_at < OffsetDateTime::now_utc().unix_timestamp() {
            return Err(AppError {
                status: StatusCode::UNAUTHORIZED,
                message: "Token Expired".into(),
            });
        }

        Ok(AuthUser { user_id: claims.id })
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    ApiContext: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApiContext::from_ref(state);

        let auth_header = parts.headers.get(AUTHORIZATION).ok_or(AppError {
            status: StatusCode::UNAUTHORIZED,
            message: "`Authorization` header is missing".into(),
        })?;

        Self::from_auth_header(auth_header, state)
    }
}
