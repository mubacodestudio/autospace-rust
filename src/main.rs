mod config;
mod handler;
mod routes;
mod schema;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::post,
    Extension,
};
use config::config::Config;
use dotenv::dotenv;
use routes::create_router;
use schema::QueryRoot;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppStatus {
    pub db: Pool<Postgres>,
    pub env: Config,
}

async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "svelte_axum_project=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        // .allow_origin("http://localhost:3000/".parse::<HeaderValue>().unwrap())
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::POST, Method::GET, Method::PUT, Method::DELETE])
        // .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT]);

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(pool.clone())
        .finish();

    let app_status = Arc::new(AppStatus {
        db: pool.clone(),
        env: config.clone(),
    });

    let app = create_router(app_status.clone())
        .route("/graphql", post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

    tracing::info!("GraphQL endpoint available at http://localhost:8000/graphql");
    tracing::info!("Server listening on http://localhost:8000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
