use crate::hash::PasswordHasher;
use crate::models::{AuthUser, CreateUser, User};
use crate::repositories::UserRepository;
use crate::Pool;
use uuid::Uuid;
use juniper::{EmptySubscription, FieldResult, RootNode};

// re-exports
pub use juniper::http;

// The requirements to execute the query statements
#[derive(Debug, Clone)]
pub struct Context {
    pool: Pool,
    pass_hasher: PasswordHasher,
}

// implement the marker trait for the Context
impl juniper::Context for Context {}

impl Context {
    pub fn new(pool: Pool, pass_hasher: PasswordHasher) -> Self {
        Context { pool, pass_hasher }
    }

    fn user_repository(&self) -> UserRepository {
        UserRepository::new(self.pool.clone())
    }
}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }

    async fn user(id: Uuid, context: &Context) -> FieldResult<User> {
        context.user_repository().get(id).await.map_err(|e| e.into())
    }

    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        context.user_repository().all().await.map_err(|e| e.into())
    }
}

pub struct Mutation {}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    pub async fn create_user(input: CreateUser, context: &Context) -> FieldResult<AuthUser> {
        context
            .user_repository()
            .create(input, &context.pass_hasher)
            .await
            .map_err(|e| e.into())
    }
}

pub type Schema =
    RootNode<'static, Query, Mutation, juniper::EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}
