#![deny(unsafe_code)]

pub use sqlx::PgPool as Pool;

pub mod error;
pub mod graphql;
pub mod hash;
pub mod models;
pub mod repositories;
