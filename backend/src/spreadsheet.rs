use std::io::Cursor;

use axum::{
    body::Body,
    http::StatusCode,
    Json,
    response::IntoResponse,
    routing::post,
    Router,
};

use shuttle_axum::ShuttleAxum;

use umya_spreadsheet::*;
use bytes::Bytes;
use writer::xlsx;

mod models {
    pub mod fundamentals;
}

use models::fundamentals::Fundamentals;

// Constants for sheet name
const SHEET_NAME: &str = "Unwritten";

async fn spreadsheet_writer(Json(inpts): Json<Fundamentals>) -> impl IntoResponse {
    // Setup spreadsheet
    let mut book = new_file();
    let _ = book.remove_sheet(0); // Remove default sheet (Sheet1)
    let _ = book.new_sheet(SHEET_NAME);

    if let Some(sheet) = book.get_sheet_by_name_mut(SHEET_NAME) {
        // Set header values
        sheet.get_cell_mut("A1").set_value("Company Name");
        sheet.get_cell_mut("A2").set_value("Primary Sector");
        sheet.get_cell_mut("A3").set_value("Primary Country");
        sheet.get_cell_mut("A4").set_value("Total Revenue");

        // Insert JSON data into spreadsheet
        sheet.get_cell_mut("B1").set_value(inputs.company_name.clone());
        sheet.get_cell_mut("B2").set_value(inputs.primary_sector.clone());
        sheet.get_cell_mut("B3").set_value(inputs.primary_country.clone());
        sheet.get_cell_mut("B4").set_value(inputs.total_revenue.clone());
    }

    // Set style
    let style =  book.get_sheet_by_name_mut(SHEET_NAME).unwrap().get_style_mut("A2");
    style.set_background_color(Color::COLOR_BLUE); // Fill color

    // Save Excel to an in-memory buffer
    let mut buffer = Cursor::new(Vec::new());
    let _ = xlsx::write_writer(&book, &mut buffer).expect("Failed to write Excel to buffer");

    // Read data from the buffer and prepare it as bytes
    let file_data = Bytes::from(buffer.into_inner());

     // Return the Excel file as response
     (StatusCode::OK, Body::from(file_data))
}


#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    /*
    NOTE: Verify the `Fundamentals` struct and ensure that the `/api/` endpoint is included in the request.
    Use this cURL command to test the `/api/` endpoint:
    
    curl http://127.0.0.1:8000/api/ \
    -H "Content-Type: application/json" \
    -d '{"company_name": "a", "primary_sector": "bb", "primary_country": "c", "total_revenue": "3"}' \
    -o model.xlsx
    */
    let router = Router::new().route("/api/", post(spreadsheet_writer));
    Ok(router.into())
}