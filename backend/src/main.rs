use axum::{routing::get, Router};
use axum::extract::Path; // path extraction

async fn extract_url(Path(twid): Path<String>) -> String {
    format!("Hello, world name: {twid}! umya-spreadsheet compiled!")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/:twid", get(extract_url));

    Ok(router.into())
}