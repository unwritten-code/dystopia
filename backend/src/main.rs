use axum::{
    body::Body, extract::Path, http::{header, HeaderValue, StatusCode}, response::IntoResponse, routing::post, Json, Router
};

use serde::{Deserialize, Serialize};
use umya_spreadsheet::*;
use bytes::Bytes;
use writer::xlsx;
use std::io::Cursor;

// used to create static files / webpage
use tower_http::services::ServeDir;

#[derive(Serialize, Deserialize)]
struct Params {
    key: String, 
    calculate: String
}

async fn xx(Json(json): Json<Params>) -> String {
    format!("Hello {0} {1}", json.key, json.calculate)
}

async fn _excel(Path(val): Path<String>) -> impl IntoResponse {    
    // TASK 1 create a simple post
    // TASK 2 send it a real json
    // TASK 3 spit that out in excel
    // TASK 4 create a post in nextJS

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
    // Serve static files from the "assets" directory under the "/static" prefix
    let static_files = Router::new().nest_service("/", ServeDir::new("assets"));
    
    
    let dynamic_route = Router::new().route("/api/", post(xx));
    let router = static_files.merge(dynamic_route);

    Ok(router.into())
}