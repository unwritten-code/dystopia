use axum::{
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get,
    body::Body,
    Router,
};

use umya_spreadsheet::*;
use bytes::Bytes;
use writer::xlsx;

use std::io::Cursor;

async fn excel_generator(Path(val): Path<String>) -> impl IntoResponse {    
    // setup spreadsheet
    let mut book = new_file();
    let sheet_name = "Unwritten";

    let _ = book.remove_sheet(0); // remove sheet1
    let _ = book.new_sheet(sheet_name);

    // insert parse `val` into spreadsheet
    book.get_sheet_by_name_mut(sheet_name)
        .unwrap()
        .get_cell_mut("B2") // cell reference
        .set_value(val.clone());

    let style =  book.get_sheet_by_name_mut(sheet_name).unwrap().get_style_mut("A2");
    style.set_background_color(Color::COLOR_BLUE); // fill color

    // save excel to an in-memory buffer
    let mut buffer = Cursor::new(Vec::new());
    let _ = xlsx::write_writer(&book, &mut buffer).expect("Failed to write Excel to buffer");

    // read data from the buffer and prepare it as bytes
    let file_data = Bytes::from(buffer.into_inner());

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