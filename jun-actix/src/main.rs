use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use jun::graphql::{
    create_schema, http::graphiql::graphiql_source, http::GraphQLRequest, Context, Schema,
};
use log::info;

async fn handle_graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    context: web::Data<Context>,
) -> HttpResponse {
    let res = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(res)
}

async fn handle_graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

fn app_config(config: &mut web::ServiceConfig) {
    let schema = create_schema();

    config
        .data(schema)
        .service(web::resource("/graphql").route(web::post().to(handle_graphql)))
        .service(web::resource("/graphiql").route(web::get().to(handle_graphiql)))
        .service(web::resource("/").route(web::get().to(|| {
            HttpResponse::PermanentRedirect()
                .set_header(actix_web::http::header::LOCATION, "/graphiql")
                .finish()
        })));
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use std::env::var;

    dotenv::dotenv().ok();
    env_logger::init();

    let host = var("SERVER_HOST").expect("SERVER_HOST is not part of env");
    let port = var("SERVER_PORT").expect("SERVER_PORT is not part of env");
    let server_url = var("SERVER_URL").expect("SERVER_URL is not part of env");
    let server_addr = format!("{}:{}", host, port);

    let database_url = var("DATABASE_URL").expect("DATABASE_URL is not part of env");
    let pool = jun::Pool::new(&database_url)
        .await
        .expect(&format!("failed to connect to database: {}", &database_url));
    let context = Context::new(pool);

    info!("Server starting at: {}", server_url);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(context.clone())
            .configure(app_config)
    })
    .bind(server_addr)?
    .run()
    .await
}
