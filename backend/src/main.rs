use axum::{
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    body::Body,
    Router,
};

// use umya_spreadsheet::*;

use bytes::Bytes;

async fn excel_generator(Path(val): Path<String>) -> impl IntoResponse {    
    // setup spreadsheet
    // let mut book = new_file();
    // let _ = book.new_sheet("Unwritten");

    // insert parse `val` into spreadsheet
    // book.get_sheet_by_name_mut("Unwritten")
    //     .unwrap()
    //     .get_cell_mut("B2") // cell reference
    //     .set_value(val.clone());

    let file_data = Bytes::from_static(include_bytes!(".././phillip-sucks.xlsx"));


    
    // setup headers for a downloadable file
     let mut headers = axum::http::HeaderMap::new();
     headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"));
     headers.insert(header::CONTENT_DISPOSITION, HeaderValue::from_str("attachment; filename=\"model.xlsx\"").unwrap());

     (StatusCode::OK, headers, Body::from(file_data))
}


#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/:val", get(excel_generator));
    
    Ok(router.into())
}

// Create a temporary path
    // let path = std::path::Path::new("./model3.xslx");
    // let _ = writer::xlsx::write(&book, path);