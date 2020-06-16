#![deny(unsafe_code)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub use sqlx::PgPool as Pool;

pub mod error;
pub mod graphql;
pub mod hash;
pub mod models;
pub mod repositories;
