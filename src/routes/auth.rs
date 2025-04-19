use axum::{
  extract::State,
  routing::post,
  Json, Router,
};
use sqlx::PgPool;

use crate::models::user::{AuthResponse, LoginUser, RegisterUser, User};
use crate::utils::hasher::{hash_password, verify_password};

#[utoipa::path(
  post,
  path = "/register",
  request_body = RegisterUser,
  responses(
      (status = 200, description = "User registered successfully", body = AuthResponse),
      (status = 400, description = "Registration failed", body = AuthResponse)
  ),
  security()
)]
async fn register(
  State(pool): State<PgPool>,
  Json(payload): Json<RegisterUser>,
) -> Json<AuthResponse> {
  let password_hash = match hash_password(&payload.password) {
      Ok(hash) => hash,
      Err(_) => {
          return Json(AuthResponse {
              success: false,
              message: "Failed to hash password".to_string(),
              user: None,
          })
      }
  };

  let api_key = User::generate_api_key();

  let result = sqlx::query_as::<_, User>(
      "INSERT INTO users (email, password_hash, api_key) VALUES ($1, $2, $3) RETURNING *"
  )
  .bind(&payload.email)
  .bind(&password_hash)
  .bind(&api_key)
  .fetch_one(&pool)
  .await;

  match result {
      Ok(user) => Json(AuthResponse {
          success: true,
          message: "Registration successful".to_string(),
          user: Some(user.to_response()),
      }),
      Err(err) => {
          eprintln!("Registration error: {}", err);
          Json(AuthResponse {
              success: false,
              message: "Registration failed. Email might already be in use.".to_string(),
              user: None,
          })
      }
  }
}


#[utoipa::path(
  post,
  path = "/login",
  request_body = LoginUser,
  responses(
      (status = 200, description = "Login successful", body = AuthResponse),
      (status = 401, description = "Login failed", body = AuthResponse)
  ),
  security()
)]
async fn login(
  State(pool): State<PgPool>,
  Json(payload): Json<LoginUser>,
) -> Json<AuthResponse> {
  let result = sqlx::query_as::<_, User>(
      "SELECT * FROM users WHERE email = $1"
  )
  .bind(&payload.email)
  .fetch_optional(&pool)
  .await;

  match result {
      Ok(Some(user)) => {
          match verify_password(&user.password_hash, &payload.password) {
              Ok(true) => Json(AuthResponse {
                  success: true,
                  message: "Login successful".to_string(),
                  user: Some(user.to_response()),
              }),
              _ => Json(AuthResponse {
                  success: false,
                  message: "Invalid email or password".to_string(),
                  user: None,
              }),
          }
      },
      Ok(None) => Json(AuthResponse {
          success: false,
          message: "Invalid email or password".to_string(),
          user: None,
      }),
      Err(err) => {
          eprintln!("Login error: {}", err);
          Json(AuthResponse {
              success: false,
              message: "Login failed".to_string(),
              user: None,
          })
      }
  }
}

pub fn routes() -> Router<PgPool> {
  Router::new()
      .route("/register", post(register))
      .route("/login", post(login))
}