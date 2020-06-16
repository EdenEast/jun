use crate::models::User;
use crate::repositories::UserRepository;
use crate::Pool;
use juniper::{EmptyMutation, EmptySubscription, FieldError, RootNode};

// re-exports
pub use juniper::http;

// The requirements to execute the query statements
#[derive(Debug, Clone)]
pub struct Context {
    pool: Pool,
}

// implement the marker trait for the Context
impl juniper::Context for Context {}

impl Context {
    pub fn new(pool: Pool) -> Self {
        Context { pool }
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

    async fn users(context: &Context) -> Result<Vec<User>, FieldError> {
        let repository = context.user_repository();
        let users = repository.all().await?;
        Ok(users)
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<Context>, juniper::EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}
