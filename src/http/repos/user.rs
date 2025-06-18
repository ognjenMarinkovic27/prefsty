use crate::http::repos::{
    error::DbError,
    model::{User, UserSafe},
};

#[derive(Debug)]
pub struct UserRepo {
    pool: sqlx::PgPool,
}

impl UserRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_by_username(&self, username: &str) -> Result<User, DbError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, password, email FROM users WHERE username = $1",
            username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::error::Error::RowNotFound => DbError::NotFound(e),
            _ => e.into(),
        })?;

        Ok(user)
    }

    pub async fn create(&self, user: User) -> Result<UserSafe, DbError> {
        let user = sqlx::query_as!(
            UserSafe,
            r#"
            INSERT INTO users (id, username, password, email)
            VALUES ($1, $2, $3, $4)
            RETURNING id, username
            "#,
            user.id,
            user.username,
            user.password,
            user.email
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}
