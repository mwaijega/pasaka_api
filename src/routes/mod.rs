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
    info(
        title = "Pasaka Swahili Bible API",
        version = "1.0.0",
        description = "🚀 **The Pasaka Swahili Bible API That’s Faster Than Your Wi-Fi** 💨📖\n\nYou know how sometimes you need something so fast, it feels like it’s already done before you even ask? Well, that's what this Swahili Bible API feels like! ⚡ Whether you’re building an app, a website, or just need the Bible in your project at the speed of light, we've got you covered. It's like a Bible, but faster than you can say 'Genesis'—and without any holy lag! ⏱️\n\n**Why is it soooo fast?**\n\n📚 **Blink-and-you-miss-it Speed:** Get any verse, chapter, or book from the Swahili Bible quicker than you can open your coffee mug in the morning.\n\n🔍 **Need something specific?** Search results pop up like magic 🧙‍♂️. You won’t even know what hit you.\n\n🌍 **Swahili, Everywhere:** It's not just fast—it’s super Swahili-friendly for everyone who wants the Word in their own language.\n\n⚡ **Ultra-Optimized:** If you like smooth, responsive apps, this is your new best friend. It’s fast enough to make your grandma say, 'Wow, that’s fast!' 😮\n\nNo more waiting for Bible verses. No more endless scrolling. Just pure, unadulterated, ultra-speedy Bible goodness right when you need it. 🚀\n\nSo, get ready to elevate your app with the ultra-fast, ultra-smooth, ultra-awesome Swahili Bible API. It's a divine experience in speed and simplicity. 🙌"
    ),
    security(
        ("api_key" = [])
    ),
)]
pub struct ApiDoc;