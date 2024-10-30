use axum::{
    extract::Path,
    response::IntoResponse,
    routing::get,
    Router,
};

use umya_spreadsheet::*;

async fn extract_url(Path(value): Path<u32>) -> impl IntoResponse {
    format!("Hello, world name: {value}! umya-spreadsheet compiled!")
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/:value", get(extract_url));
    
    // setup spreadsheet
    let mut book = new_file();
    let _ = book.new_sheet("SheetPhillip");

    // temp spreadsheet storage location
    // let mut temp_file

    Ok(router.into())
}