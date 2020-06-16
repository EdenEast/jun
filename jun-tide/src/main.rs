use jun::graphql::{create_schema, http, Context, Schema};
use tide::Body;
use tide::Redirect;
use tide::Server;
use tide::StatusCode;
use tide::{Request, Response};

pub struct State {
    pub context: Context,
    pub schema: Schema,
}

impl State {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            schema: create_schema(),
        }
    }
}

async fn handle_graphql(mut req: Request<State>) -> tide::Result {
    let query: http::GraphQLRequest = req
        .body_json()
        .await
        .expect("be able to deserialize graphql request");

    let context = &req.state().context;
    let schema = &req.state().schema;
    let response = query.execute(schema, context).await;
    let status = if response.is_ok() {
        StatusCode::Ok
    } else {
        StatusCode::BadRequest
    };

    let mut res = Response::new(status);
    res.set_body(Body::from_json(&response)?);
    Ok(res)
}

async fn handle_graphiql(_: Request<State>) -> tide::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.set_body(http::graphiql::graphiql_source("/graphql", None));
    res.set_content_type(tide::http::mime::HTML);
    Ok(res)
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    tide::log::start();

    let host = std::env::var("SERVER_HOST").expect("SERVER_HOST is not part of env");
    let port = std::env::var("SERVER_PORT").expect("SERVER_PORT is not part of env");
    let _server_url = std::env::var("SERVER_URL").expect("SERVER_URL is not part of env");
    let server_addr = format!("{}:{}", host, port);

    let state = State::new();

    let mut app = Server::with_state(state);
    app.middleware(tide::log::LogMiddleware::new());
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    app.listen(server_addr).await?;

    Ok(())
}
