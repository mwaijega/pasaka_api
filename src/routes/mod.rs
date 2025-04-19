use utoipa::OpenApi;

pub mod auth;
pub mod bible;

#[derive(OpenApi)]
#[openapi(
    paths(
        bible::get_books,
        bible::get_book,
        bible::get_chapter,
        bible::get_verse,
        bible::search_bible,
        auth::register,
        auth::login,
    ),
    components(
        schemas(
            crate::models::bible::BibleRoot,
            crate::models::bible::Book,
            crate::models::bible::Chapter,
            crate::models::bible::Verse,
            crate::models::bible::SearchResult,
            crate::models::bible::BibleResponse<Vec<crate::models::bible::Book>>,
            crate::models::bible::ErrorResponse,
            crate::models::bible::SearchQuery,
            crate::models::user::RegisterUser,
            crate::models::user::LoginUser,
            crate::models::user::UserResponse,
            crate::models::user::AuthResponse,
        )
    ),
    tags(
        (name = "Bible API", description = "Bible endpoints"),
        (name = "Auth API", description = "Authentication endpoints")
    ),
    info(
        title = "Swahili Bible API",
        version = "1.0.0",
        description = "API for accessing Swahili Bible text"
    ),
    security(
        ("api_key" = [])
    ),
)]
pub struct ApiDoc;