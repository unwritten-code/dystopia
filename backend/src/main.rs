use axum::http::StatusCode;
use axum::{
    response::IntoResponse, routing::post, Json, Router
};

use shuttle_axum::ShuttleAxum;

use polars::prelude::*;
use polars_io::SerReader;
use polars_io::prelude::{JsonReader, JsonFormat};

use std::io::Cursor;
use serde_json::Value;

mod models{
    pub mod fundamentals;
}


async fn polars_end_point(Json(inputs): Json<Value>) -> impl IntoResponse {

    // Convert JSON data to a string and wrap it in a Cursor
    let json_data = serde_json::to_string(&inputs).expect("Failed to serialize JSON");
 
     // Convert JSON to Polars DataFrame
     let df = JsonReader::new(Cursor::new(json_data))
         .with_json_format(JsonFormat::Json)
         .finish()
         .expect("Failed to create DataFrame");

    // Convert to LazyFrame to perform expression-based transformations
    let df = df.lazy(); 

    let df = df.with_column((col("total_revenue") * col("sector_rev_share")).alias("sector_rev_share"));
    let df = df.with_column((col("total_revenue") * col("country_rev_share")).alias("country_rev_share"));
    
    // Execute the lazy operation
    let df = df.collect(); 
    
    print!{"{:?}", df}

    return StatusCode::OK
}


#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {

    /*
    curl http://127.0.0.1:8000/polars_end_point/ \
    -H "Content-Type: application/json" \
    -d '{"company_name": "ACME Corp", "primary_sector": "Oil and Gas", "total_revenue": 999, "revenue_by_country": [{ "search_term": "FRA", "value": 0.40, "key": "abc" }, { "search_term": "GBR", "value": 0.60, "key": "def" }]}' 
    */

    /* Flattened structure
    curl http://127.0.0.1:8000/polars_end_point/ \
    -H "Content-Type: application/json" \
    -d '[
        {
          "company_name": "ACME Corp",
          "primary_sector": "Oil and Gas",
          "total_revenue": 999,
          "country": "FRA",
          "country_rev_share": 0.40,
          "sector_rev_share": 0.20
        },
        {
          "company_name": "ACME Corp",
          "primary_sector": "Oil and Gas",
          "total_revenue": 999,
          "country": "GBR",
          "country_rev_share": 0.60,
          "sector_rev_share": 0.80
        }
      ]'
     */

    let router = Router::new().route("/polars_end_point/", post(polars_end_point));

    Ok(router.into())
}