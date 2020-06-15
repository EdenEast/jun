use chrono::naive::NaiveDateTime;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl User {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn image(&self) -> Option<&str> {
        self.image.as_deref()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    pub fn updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AuthToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, GraphQLInputObject)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
