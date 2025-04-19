use axum::{
  extract::{Path,State},
  routing::{get,post},
  Json,Router, 
};
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;
use std::fs;

use crate::models::bible::{
  BibleResponse,
  BibleRoot,
  Book,
  Chapter,
  SearchQuery,
  SearchResult,
  Verse,
  ErrorResponse
};



pub struct BibleData {
  books: Vec<Book>,
}

impl BibleData {
  pub fn load() -> Result<Self,Box<dyn std::error::Error>> {
    let file_path ="src/data/swahili_bible.json";
    let data = fs::read_to_string(file_path)?;

    let bible_root: BibleRoot = serde_json::from_str(&data)?;
    Ok(Self {books: bible_root.books})
  }

  fn get_books(&self) -> &Vec<Book>{
    &self.books
  }
  fn get_book(&self,book_id: &str) -> Option<&Book>{
    self.books.iter().find(|b| b.id == book_id)
  }

  fn search(&self,query:&str) -> Vec<SearchResult>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for book in &self.books {
      for chapter in &book.chapters {
        for verse in &chapter.verses {
          if verse.text.to_lowercase().contains(&query){
            results.push(SearchResult {
              book: book.name.clone(),
              chapter: chapter.chapter.clone(),
              verse: verse.text.clone(),
              text: verse.text.clone(),
            });
          }
        }
      }
    }
    results
  }
}

type AppState = Arc<(PgPool, BibleData)>;

#[utoipa::path(
  get,
  path="/books",
  responses(
    (status=200,description = "List of all book",body = BibleResponse<Vec<Book>>),
    (status =401,description = "Unauthorized",body = ErrorResponse)

  ),
  security(
    ("api_key" =[])
  )
)]

async fn get_books(State(state):State<AppState>,) -> Json<BibleResponse<Vec<Book>>>{
  let bible_data =&state.1;

  Json(BibleResponse{
    success: true,
    data:bible_data.get_books().clone(),
  })
}

#[utoipa::path(
  get,
  path = "/books/{book_id}",
  params(
      ("book_id" = String, Path, description = "Book ID")
  ),
  responses(
      (status = 200, description = "Book details", body = BibleResponse<Book>),
      (status = 404, description = "Book not found", body = ErrorResponse),
      (status = 401, description = "Unauthorized", body = ErrorResponse)
  ),
  security(
      ("api_key" = [])
  )
)]
async fn get_book(
  State(state):State<AppState>,
  Path(book_id):Path<String>,

) -> Json<Value> {
  let bible_data = &state.1;

  match bible_data.get_book(&book_id){
    Some(book) => Json(serde_json::json!({
      "success":true,
      "data":book,
    })),
    None => Json(serde_json::json!({
      "sucess":false,
      "error":format!("Book '{}' not found",book_id)
    })),
  }
}

#[utoipa::path(
  get,
  path="/book/{book_id}/{chapter}",
  params(
    
    ("book_id"=String, Path, description="Book ID"),
    ("chapter"=String, Path, description="Chapter number")

  ),
  responses(
    (status = 200, description="Chapter details",body =BibleResponse<Chapter>),
    (status = 404,description="Book or Chapter not found",body=ErrorResponse),
    (status = 401,description="Unauthorized",body=ErrorResponse),
  ),
  security(
    ("api_key"=[])
  )
)]
async fn get_chapter(
  State(state): State<AppState>,
  Path((book_id,chapter)): Path<(String,String)>,
) -> Json<Value>{
  let bible_data = &state.1;
  
  if let  Some(book) = bible_data.get_book(&book_id){
    if let Some(chap) = book.chapters.iter().find(|c| c.chapter == chapter){
      return Json(serde_json::json!({
        "success":true,
        "data":chap,
      }));
    }
    return Json(serde_json::json!({
      "success":false,
      "error":format!("Chapter {} not found in book '{}",chapter,book_id)
    }));

  }
  Json(serde_json::json!({
    "success":false,
    "error":format!("Book '{}' not found",book_id)
  }))
}

#[utoipa::path(
  get,
  path = "/books/{book_id}/{chapter}/{verse}",
  params(
      ("book_id" = String, Path, description = "Book ID"),
      ("chapter" = String, Path, description = "Chapter number"),
      ("verse" = String, Path, description = "Verse number")
  ),
  responses(
      (status = 200, description = "Verse details", body = BibleResponse<Verse>),
      (status = 404, description = "Book, chapter, or verse not found", body = ErrorResponse),
      (status = 401, description = "Unauthorized", body = ErrorResponse)
  ),
  security(
      ("api_key" = [])
  )
)]
async fn get_verse(
  State(state): State<AppState>,
  Path((book_id, chapter, verse)): Path<(String, String, String)>,
) -> Json<Value> {
  let bible_data = &state.1;
  
  if let Some(book) = bible_data.get_book(&book_id) {
      if let Some(chap) = book.chapters.iter().find(|c| c.chapter == chapter) {
          if let Some(v) = chap.verses.iter().find(|v| v.verse == verse) {
              return Json(serde_json::json!({
                  "success": true,
                  "data": v
              }));
          }
          
          return Json(serde_json::json!({
              "success": false,
              "error": format!("Verse {} not found in chapter {} of book '{}'", verse, chapter, book_id)
          }));
      }
      
      return Json(serde_json::json!({
          "success": false,
          "error": format!("Chapter {} not found in book '{}'", chapter, book_id)
      }));
  }
  
  Json(serde_json::json!({
      "success": false,
      "error": format!("Book '{}' not found", book_id)
  }))
}

#[utoipa::path(
  post,
  path = "/search",
  request_body = SearchQuery,
  responses(
      (status = 200, description = "Search results", body = BibleResponse<Vec<SearchResult>>),
      (status = 401, description = "Unauthorized", body = ErrorResponse)
  ),
  security(
      ("api_key" = [])
  )
)]
async fn search_bible(
  State(state): State<AppState>,
  Json(payload): Json<SearchQuery>,
) -> Json<BibleResponse<Vec<SearchResult>>> {
  let bible_data = &state.1;
  let results = bible_data.search(&payload.query);
  
  Json(BibleResponse {
    success: true,
    data: results,
  })
}

pub fn routes() -> Router<AppState> {
  Router::new()
    .route("/api/bible/books", get(get_books))
    .route("/api/bible/books/{book_id}", get(get_book))          
    .route("/api/bible/books/{book_id}/{chapter}", get(get_chapter)) 
    .route("/api/bible/books/{book_id}/{chapter}/{verse}", get(get_verse))  
    .route("/api/bible/search", post(search_bible))
}