use axum::{
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

use umya_spreadsheet::*;

async fn excel_generator(Path(val): Path<String>) -> impl IntoResponse {
    // format!("Hello, world name: {value}! umya-spreadsheet compiled!");
    // setup spreadsheet
    let mut book = new_file();
    let _ = book.new_sheet("Unwritten");

    // Insert `val` into a specific cell in the "Unwritten" sheet
    book.get_sheet_by_name_mut("Unwritten")
        .unwrap()
        .get_cell_mut("B2") // cell reference "B2"
        .set_value(val.clone());


    // store in a memory buffer, excel is binary so store as u8
    let buffer: Vec<u8> = Vec::new();

    // setup headers for a downloadable file
     let mut headers = axum::http::HeaderMap::new();
     headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"));
     headers.insert(header::CONTENT_DISPOSITION, HeaderValue::from_str("attachment; filename=\"model.xlsx\"").unwrap());

     (StatusCode::OK, headers, buffer)
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/:val", get(excel_generator));
    
    Ok(router.into())
}