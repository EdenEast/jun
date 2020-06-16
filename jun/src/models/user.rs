use chrono::naive::NaiveDateTime;
use juniper::GraphQLInputObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, juniper::GraphQLObject)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[graphql(skip)]
    pub password_hash: String,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow, juniper::GraphQLObject)]
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
