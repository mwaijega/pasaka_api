use serde::{Deserialize,Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug,FromRow,Serialize)]
pub struct User {
  pub id: i32,
  pub email: String,
  pub password_hash: String,
  pub api_key: String,
  pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug,Deserialize,ToSchema)]
pub struct RegisterUser {
  pub email: String,
  pub password: String,

}

#[derive(Debug,Deserialize,ToSchema)]
pub struct LoginUser {
  pub email: String,
  pub password: String,
  
}

#[derive(Debug,Serialize,ToSchema)]
pub struct UserResponse{
  pub id: i32,
  pub email: String,
  pub api_key: String,

}

#[derive(Debug,Serialize,ToSchema)]
pub struct AuthResponse{
  pub success: bool,
  pub message: String,
  pub user: Option<UserResponse>,
}

impl User{
  pub fn generate_api_key() -> String{
    
    format!("safina_{}",Uuid::new_v4().to_string())
  }

  pub fn to_response(&self) -> UserResponse{
    UserResponse{
      id: self.id,
      email: self.email.clone(),
      api_key: self.api_key.clone(),
    }
  }
}




