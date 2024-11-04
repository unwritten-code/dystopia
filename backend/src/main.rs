use axum::{
    body::Body, http::StatusCode, response::IntoResponse, routing::post, Json, Router
};

//https://www.mongodb.com/docs/drivers/rust/current/usage-examples/findOne/#std-label-rust-find-one-usage
use mongodb::{bson::doc, Client, Collection};

use serde::{Deserialize, Serialize};
use umya_spreadsheet::*;
use bytes::Bytes;
use writer::xlsx;
use std::{env, io::Cursor};
use dotenvy::dotenv;

// used to create static files / webpage
use tower_http::services::ServeDir;



#[derive(Serialize, Deserialize)]
struct PostParams {
    company_name: String,
    primary_sector: String,
    primary_country: String,
    total_revenue: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct NatureBySector{
    nature_risk: String,
    utics: String,
    value: String,
    materiality: String
    
}

async fn _hello_word(Json(json): Json<PostParams>) -> String {
    format!("POST returns key: {0}, value: {1}", json.company_name, json.total_revenue)
}


async fn json2excel(Json(json): Json<PostParams>) -> impl IntoResponse {
    const SHEET_NAME: &str = "Unwritten";

    // setup spreadsheet
    let mut book = new_file();
    let _ = book.remove_sheet(0); // remove sheet1
    let _ = book.new_sheet(SHEET_NAME);

    if let Some(sheet) = book.get_sheet_by_name_mut(SHEET_NAME) {
        sheet.get_cell_mut("A1").set_value("Company Name");
        sheet.get_cell_mut("A2").set_value("Primary Sector");
        sheet.get_cell_mut("A3").set_value("Primary Country");
        sheet.get_cell_mut("A4").set_value("Total Revenue");
        // insert json into spreadsheet
        sheet.get_cell_mut("B1").set_value(json.company_name.clone());
        sheet.get_cell_mut("B2").set_value(json.primary_sector.clone());
        sheet.get_cell_mut("B3").set_value(json.primary_country.clone());
        sheet.get_cell_mut("B4").set_value(json.total_revenue.clone());
    }

    // setup style
    let style =  book.get_sheet_by_name_mut(SHEET_NAME).unwrap().get_style_mut("A2");
    style.set_background_color(Color::COLOR_BLUE); // fill color

    // save excel to an in-memory buffer
    let mut buffer = Cursor::new(Vec::new());
    let _ = xlsx::write_writer(&book, &mut buffer).expect("Failed to write Excel to buffer");

    // read data from the buffer and prepare it as bytes
    let file_data = Bytes::from(buffer.into_inner());

     // return file_data
     (StatusCode::OK, Body::from(file_data))
}



async fn load_spectacle() -> mongodb::error::Result<Option<NatureBySector>> {
    // load .env file into the environment
    dotenv().ok();

    let uri = env::var("MONGODB_URI").expect("Environment variable MY_ENV_VAR not set");

    let client =Client::with_uri_str(uri).await?;

    let my_coll: Collection<NatureBySector> = client
        .database("delphi-dev")
        .collection("nature_by_sector");
    
    let result = my_coll.find_one(doc! { "nature_risk": "water_availability" }).await?;
    Ok(result)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {

    // mongodb test
    // let data = load_spectacle().await;
    // print!("{:?}", data);


    // create a static page for documentation
    let static_files = Router::new().nest_service("/", ServeDir::new("assets"));
    
    // routes
    let dynamic_route = Router::new().route("/api/", post(json2excel));
    let router = static_files.merge(dynamic_route);


    Ok(router.into())
}