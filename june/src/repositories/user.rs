use sqlx::PgPool;
use uuid::Uuid;

use crate::error::Error;
use crate::hash::PasswordHasher;
use crate::models::{AuthToken, CreateUser, User};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }

    pub async fn get(&self, id: Uuid) -> Result<User, Error> {
        let user = sqlx::query_as!(User, "select * from profile where id = $1", id)
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    pub async fn all(&self) -> Result<Vec<User>, Error> {
        let users = sqlx::query!("select * from profile")
            .fetch_all(&self.pool)
            .await?;

        Ok(users
            .into_iter()
            .map(|rec| User {
                id: rec.id,
                username: rec.username,
                email: rec.email,
                password: rec.password,
                image: rec.image,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
            })
            .collect())
    }

    pub async fn create(
        &self,
        input: CreateUser,
        hasher: &PasswordHasher,
    ) -> Result<AuthToken, Error> {
        let hash = hasher.encode(&input.password).await;

        let user = sqlx::query_as!(
            User,
            r#"
            insert into profile (username, email, password)
            values ($1, $2, $3)
            returning *"#,
            &input.username,
            &input.email,
            &hash,
        )
        .fetch_one(&self.pool)
        .await?;

        let auth_token = sqlx::query_as!(
            AuthToken,
            r#"
            insert into auth_token (user_id)
            values($1)
            returning *"#,
            user.id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(auth_token)
    }
}
