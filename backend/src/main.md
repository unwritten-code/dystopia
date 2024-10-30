use axum::{routing::get, Router};
use axum::extract::Path; // path extraction
use umya_spreadsheet::*;
// use axum::{routing::{get, post}, Router, Json};
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// struct Payload {
//     twid: String,
//     cost: u32
// }

// async fn extract_url(
//     Path(id): Path<String>,
//     // Json(json): Json<Payload>
// ) -> String {
//     format!("hello, world: {id}!")
// }




async fn simple_extract_url(Path(value): Path<u32>) -> String {
    format!("Hello, world name: {value}! umya-spreadsheet compiled!")
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/:value", get(simple_extract_url));

    let mut book = new_file();
    let _ = book.new_sheet("SheetPhillip");

    Ok(router.into())
}