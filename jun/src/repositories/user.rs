use uuid::Uuid;

use crate::{
    error::Error,
    hash::PasswordHasher,
    models::{AuthToken,AuthUser, CreateUser, User},
    Pool,
};

pub struct UserRepository {
    pool: Pool,
}

impl UserRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
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
                password_hash: rec.password_hash,
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
    ) -> Result<AuthUser, Error> {
        let hash = hasher.encode(&input.password).await;

        let user = sqlx::query_as!(
            User,
            r#"
            insert into profile (username, email, password_hash)
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

        Ok(AuthUser {
            user_id: user.id,
            token_id: auth_token.id,
            token: auth_token.token,
            username: user.username,
            email: user.email,
            image: user.image
        })
    }
}
