use axum::{
  body::Body,
  extract::State,
  http::{Request, StatusCode},
  middleware::Next,
  response::Response,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::models::bible::ErrorResponse;

// The actual middleware function
pub async fn require_api_key<S>(
  State(state): State<Arc<(PgPool, S)>>,
  request: Request<Body>,
  next: Next,
) -> Result<Response, StatusCode>
where
  S: Send + Sync + 'static,
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
          let error = ErrorResponse {
              success: false,
              error: "API key is required".to_string(),
          };
          
          return Ok(Response::builder()
              .status(StatusCode::UNAUTHORIZED)
              .header("Content-Type", "application/json")
              .body(Body::from(
                  serde_json::to_string(&error).unwrap(),
              ))
              .unwrap());
      }
  };

  // Get a reference to just the pool from the Arc tuple
  let pool = &state.0;
  
  let result = sqlx::query!("SELECT id FROM users WHERE api_key = $1", api_key)
      .fetch_optional(pool)
      .await;

  match result {
      Ok(Some(_)) => {
          // API key is valid, proceed to the next middleware/handler
          Ok(next.run(request).await)
      }
      Ok(None) => {
          // API key not found
          let error = ErrorResponse {
              success: false,
              error: "Invalid API key".to_string(),
          };
          
          Ok(Response::builder()
              .status(StatusCode::UNAUTHORIZED)
              .header("Content-Type", "application/json")
              .body(Body::from(
                  serde_json::to_string(&error).unwrap(),
              ))
              .unwrap())
      }
      Err(_) => {
          // Database error
          let error = ErrorResponse {
              success: false,
              error: "Internal server error".to_string(),
          };
          
          Ok(Response::builder()
              .status(StatusCode::INTERNAL_SERVER_ERROR)
              .header("Content-Type", "application/json")
              .body(Body::from(
                  serde_json::to_string(&error).unwrap(),
              ))
              .unwrap())
      }
  }
}