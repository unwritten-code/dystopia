use axum::{
    response::IntoResponse,
    routing::get,
    Router,
    // body::Body,
    http::StatusCode
};

use shuttle_axum::ShuttleAxum;
use mongodb::{bson::{doc, Document}, Client, Collection}; // Alias mongodb::Cursor to MongoCursor
use polars::prelude::*;
use std::env;
use dotenvy::dotenv;

mod models{
    pub mod spectacle;
}
use models::spectacle::UParams;


// #[derive(Serialize)]
// struct ApiResponse<T> {
//     success: bool,
//     data: Option<T>,
//     message: Option<String>,
// }

async fn uparams_df(
        mongo_client: Client,
        uparams_query: Document,
    ) -> Result<DataFrame, PolarsError>  {
    // select uparams collection from database
    let uparams: Collection<UParams> = mongo_client
        .database("delphi-dev")
        .collection("uparams");

    let mut cursor = uparams.find(uparams_query).await.expect("Failed to fetch documents");

    // Initialize vectors to store each column's data
    let mut col_iso3: Vec<String> = Vec::new();
    let mut col_scenario: Vec<String> = Vec::new();
    let mut col_utics: Vec<String> = Vec::new();
    let mut col_year: Vec<i32> = Vec::new();
    let mut col_value: Vec<f64> = Vec::new();
    let mut col_delphi_financial_var: Vec<String> = Vec::new();


    while cursor.advance().await.expect("Failed to advance cursor") {
        match cursor.deserialize_current() {
            Ok(doc) => {
                col_iso3.push(doc.iso3);
                col_scenario.push(doc.scenario);
                col_utics.push(doc.utics);
                col_year.push(doc.year);
                col_value.push(doc.value);
                col_delphi_financial_var.push(doc.delphi_financial_var);
            },
            Err(e) => eprintln!("Failed to deserialize document: {}", e),
        }
    };

    // Create a Series from vectors and build the DataFrame
    let df = DataFrame::new(vec![
        Column::new("iso3".into(), col_iso3),
        Column::new("scenario".into(), col_scenario),
        Column::new("utics".into(), col_utics),
        Column::new("year".into(), col_year),
        Column::new("value".into(), col_value),
        Column::new("delphi_financial_var".into(), col_delphi_financial_var),
    ])?;

    Ok(df)
}


async fn my_get() -> impl IntoResponse {
    // Load .env file into the environment
    dotenv().ok();
    
    // Reusable connection to MongoDB
    let mongo_uri = env::var("MONGODB_URI").expect("Environment variable MONGODB_URI not set");
    let mongo_client = Client::with_uri_str(&mongo_uri).await.expect("Failed to initialize MongoDB client");

    let uparams_query: Document = doc! {
        "iso3": { "$in": ["GBR"] }
      };

    let df = uparams_df(mongo_client, uparams_query).await.expect("Failed to fetch data");
    print!("{:#?}", df);

    // do maths

    // convert DataFrame to JSON. Use Polars write to json into memory buffer. Return as a body

    // Return the JSON as a response
    return StatusCode::OK
}


#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    
    // mongoDB
    let router = Router::new().route("/mdb/", get(my_get));
    
    Ok(router.into())
}