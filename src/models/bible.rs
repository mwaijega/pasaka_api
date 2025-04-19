use serde::{Deserialize,Serialize};
use utoipa::ToSchema;

#[derive(Debug,Serialize,Deserialize,ToSchema)]
pub struct BibleRoot{
  #[serde(rename ="BIBLEBOOK")]
  pub books:Vec<Book>,
}

#[derive(Debug,Clone,Serialize,Deserialize,ToSchema)]
pub struct Book {
  #[serde(rename = "book_number")]
  pub id: String,
  #[serde(rename = "book_name")]
  pub name: String,
  #[serde(rename = "CHAPTER")]
  pub chapters: Vec<Chapter>,
}

#[derive(Debug,Serialize,Deserialize,ToSchema)]
pub struct Chapter {
  #[serde(rename="chapter_number")]
  pub chapter: String,
  #[serde(rename="VERSES")]
  pub verses: Vec<Verse>,

}
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Verse {
    #[serde(rename = "verse_number")]
    pub verse: String,
    #[serde(rename = "verse_text")]
    pub text: String,
}

#[derive(Debug,Serialize,Deserialize,ToSchema)]
pub struct SearchResult{
  pub book:String,
  pub chapter:String,
  pub verse: String,
  pub text: String,
}

#[derive(Debug,Serialize,ToSchema)]
pub struct BibleResponse<T>{
  pub success: bool,
  pub data: T,

}

#[derive(Debug,Serialize,ToSchema)]
pub struct ErrorResponse {
  pub success:bool,
  pub error: String,
}

#[derive(Debug,Deserialize,ToSchema)]
pub struct SearchQuery {
  pub query: String,
}
