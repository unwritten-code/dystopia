use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router
};

use serde_json::{Value, json};
use shuttle_axum::ShuttleAxum;
use mongodb::{bson::{doc, Document}, Client, Cursor as MCursor};

use polars_io::SerReader;
use polars_io::prelude::{JsonReader, JsonFormat};

use std::io::Cursor;
use std::env;
use dotenvy::dotenv;

mod models{
    pub mod spectacle;
}

use models::spectacle::UParams;

// use StreamExt from futures to asynchronously collect the cursor items.
use futures::StreamExt;



async fn connect_mdb(Path(country): Path<String>) -> impl IntoResponse {
    dotenv().ok(); // load environment variables

    // Connect to MongoDB
    let mongo_uri = env::var("MONGODB_URI").expect("Environment variable MONGODB_URI not set");
    let mongo_client = Client::with_uri_str(&mongo_uri).await.expect("Failed to initialize MongoDB client");

    // Construct the query document for MongoDB
    let query_filter: Document = doc! {
        "iso3": { "$in": [country.clone()] }
    };
    
    // Access the "uparams" collection in the "delphi-dev" database
    let collection = mongo_client
        .database("delphi-dev")
        .collection("uparams");

    // Execute the find operation with the specified filter and await the results
    let cursor: MCursor<UParams> = collection
        .find(query_filter)
        .await
        .expect("Failed to fetch documents");

    // Cursor<T> != Json
    // https://docs.rs/mongodb/3.1.0/mongodb/struct.Cursor.html

    /* TERRIBLE CODE STARTS HERE */

    // Collect the documents into a JSON array
    let json_array: Value = serde_json::json!(cursor
        .map(|doc_result| {
            doc_result
                .map(|doc| serde_json::to_value(doc).expect("Failed to convert to JSON"))
                .unwrap_or_else(|_| json!({ "error": "Failed to fetch document" }))
        })
        .collect::<Vec<_>>()
        .await);

    /* Terrible Code Starts Here*/
    // let df = cursor_to_df(cursor).await;

    let df = JsonReader::new(Cursor::new(json_array.to_string()))
        .with_json_format(JsonFormat::Json)
        .finish();

    print!("{:?}", df);

    return StatusCode::OK
}



#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {

    // example query http://127.0.0.1:8000/api/GBR
    let router = Router::new().route("/api/:country", get(connect_mdb));

    Ok(router.into())
}