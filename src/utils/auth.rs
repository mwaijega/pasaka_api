use axum::{
  extract::State,
  http::{Request, StatusCode},
  middleware::Next,
  response::Response,
  Json,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::bible::ErrorResponse;

// Make a middleware function to check API key
pub fn require_api_key<B>() -> axum::middleware::from_fn_with_state<Arc<(PgPool, impl Send + Sync + 'static)>, fn(Request<B>, State<Arc<(PgPool, impl Send + Sync + 'static)>>, Next<B>) -> Result<Response, StatusCode>> 
where
  B: Send + 'static,
{
  // This creates a middleware that has access to the State
  axum::middleware::from_fn_with_state(state, api_key_middleware)
}

// The middleware implementation
async fn api_key_middleware<S, B>(
  request: Request<B>,
  state: State<Arc<(PgPool, S)>>,
  next: Next<B>,
) -> Result<Response, StatusCode>
where
  S: Send + Sync + 'static,
  B: Send + 'static,
{
  // Get API key from header
  let api_key = request
      .headers()
      .get("x-api-key")
      .and_then(|header| header.to_str().ok())
      .map(|s| s.to_string());
  
  // Check if API key is present
  let api_key = match api_key {
      Some(key) => key,
      None => {
          let json = Json(ErrorResponse {
              success: false,
              error: "API key is required".to_string(),
          });
          return Ok(Response::builder()
              .status(StatusCode::UNAUTHORIZED)
              .body(axum::body::Full::from(
                  serde_json::to_vec(&json).unwrap(),
              ))
              .unwrap());
      }
  };
  
  // Verify API key against database
  let pool = &state.0;
  let result = sqlx::query!("SELECT id FROM users WHERE api_key = $1", api_key)
      .fetch_optional(pool)
      .await;
  
  match result {
      Ok(Some(_)) => {
          // API key is valid, proceed to the next middleware/handler
          let response = next.run(request).await;
          Ok(response)
      }
      Ok(None) => {
          // API key not found
          let json = Json(ErrorResponse {
              success: false,
              error: "Invalid API key".to_string(),
          });
          Ok(Response::builder()
              .status(StatusCode::UNAUTHORIZED)
              .body(axum::body::Full::from(
                  serde_json::to_vec(&json).unwrap(),
              ))
              .unwrap())
      }
      Err(_) => {
          // Database error
          let json = Json(ErrorResponse {
              success: false,
              error: "Internal server error".to_string(),
          });
          Ok(Response::builder()
              .status(StatusCode::INTERNAL_SERVER_ERROR)
              .body(axum::body::Full::from(
                  serde_json::to_vec(&json).unwrap(),
              ))
              .unwrap())
      }
  }
}