use axum::{
    http::StatusCode, response::IntoResponse, routing::post, Json, Router
};

use shuttle_axum::ShuttleAxum;

use serde_json::Value;

use polars_io::SerReader;
use polars_io::prelude::{JsonReader, JsonFormat};

use std::io::Cursor;

mod models{
    pub mod fundamentals;
}


async fn send_it(Json(inputs): Json<Value>) -> impl IntoResponse {

    // Convert JSON data to a string and wrap it in a Cursor
    let json_data = serde_json::to_string(&inputs).expect("Failed to serialize JSON");
 
     // Convert to Polars DataFrame
     let df = JsonReader::new(Cursor::new(json_data))
         .with_json_format(JsonFormat::Json)
         .finish()
         .expect("Failed to create DataFrame");

    print!{"{:?}", df}

    let df = df.explode(["revenue_by_country"]);

    print!{"{:?}", df}

    return StatusCode::OK
}


#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {

    /*
    curl http://127.0.0.1:8000/send_it/ \
    -H "Content-Type: application/json" \
    -d '{"company_name": "ACME Corp", "primary_sector": "Oil and Gas", "total_revenue": 999, "revenue_by_country": [{ "search_term": "FRA", "value": 0.40, "key": "abc" }, { "search_term": "GBR", "value": 0.60, "key": "def" }]}' 
  */

    let router = Router::new().route("/send_it/", post(send_it));

    Ok(router.into())
}