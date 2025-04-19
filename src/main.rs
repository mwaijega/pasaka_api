use axum::{routing::{get,post},Router,};
use axum::extract::State;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any,CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;


mod models;
mod routes;
mod utils;

#[derive(Clone)]
pub struct BibleData { /* ... */ }

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    bible_data: BibleData,
}




#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{

  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await?;
  let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

  let bible_data = BibleData::new();

  let state = AppState { pool, bible_data };

  let app = Router::new()
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json",routes::ApiDoc::openapi()))
    .nest("/api/bible",routes::bible::routes())
    .nest("/api/auth",routes::auth::routes())
    .layer(cors)
    .with_state(pool);
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app).await.unwrap();

  Ok(())
}








