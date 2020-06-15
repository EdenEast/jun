use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub use juniper::http;

// The requirements to execute the query statements
pub struct Context {}

// implement the marker trait for the Context
impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        Context {}
    }
}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn api_version() -> &str {
        "1.0"
    }
}

pub type Schema =
    RootNode<'static, Query, EmptyMutation<Context>, juniper::EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new())
}
