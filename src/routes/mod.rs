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
        ),   
    ),
    info(
        title = "Pasaka Swahili Bible API",
        version = "1.0.0",
        description = "
        📦 Note:  Use the API via `curl` for testing, add -H and your API key.
        
        You can test without Swagger using the command below. Just remember to add your `x-api-key`.
        
        ```bash
        curl -H \"x-api-key: pasaka_api_7a782fcd-da06-4558-xxxxx\" \
        https://pasaka.4insec.com/books
        ```
        note that i dont put `/` at the end of the url, because it will not work.

        so let look another example 
        
        want whole firstbook, it will be like this 
        ```bash
        curl -H \"x-api-key: pasaka_api_7a782fcd-da06-4558-xxxxx\" \
        https://pasaka.4insec.com/books/1
        ```
        again notice that i dont put `/` at the end of the url, because it will not work. 


        and how is fast 🤫🤫 just test it... "),
    security(
        ("api_key" = [])
    ),



 )]
pub struct ApiDoc;