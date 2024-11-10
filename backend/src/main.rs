use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router
};

use serde_json::Value;
use shuttle_axum::ShuttleAxum;
use mongodb::{bson::{doc, Document}, Client, Collection};

use polars::prelude::*;
use polars_io::SerReader;
use polars_io::prelude::{JsonReader, JsonFormat};

use std::io::Cursor;
use std::env;

use dotenvy::dotenv;

mod models{
    pub mod spectacle;
    pub mod fundamentals;
}

use models::spectacle::UParams;


async fn _uparams_df(
        mongo_client: Client,
        uparams_query: Document,
    ) -> Result<DataFrame, PolarsError>  {
    // select uparams collection from database
    let uparams: Collection<UParams> = mongo_client
        .database("delphi-dev")
        .collection("uparams");

    // Cursor<T> != Json
    // https://docs.rs/mongodb/3.1.0/mongodb/struct.Cursor.html
    let mut result = uparams.find(uparams_query).await.expect("Failed to fetch documents");

    let mut vector = Vec::new();

    // Iterate through documents and store in a Vector
    while result.advance().await.expect("Failed to advance result") {
        match result.deserialize_current() {
            Ok(document) => {
                // Push successfully deserialized document to vector
                vector.push(document);
            },
            Err(e) => {
                // Handle errors
                eprintln!("Failed to deserialize document: {:?}", e);
            }
        }
    }

    // Convert the vector to JSON
    let json_data = serde_json::to_string(&vector).expect("Failed to serialize vector to JSON");

    //https://docs.pola.rs/api/rust/dev/polars_io/json/index.html
    // Load a Json into a Dataframe with a Cursor
    let df = JsonReader::new(Cursor::new(json_data))
        .with_json_format(JsonFormat::Json)
        .finish()?;
 
    Ok(df)
}


async fn _get_mongodb(Path((country, sector)): Path<(String, String)>) -> impl IntoResponse {

    // unpack uri
    let query: Document = doc! {
        "iso3": { "$in": [country.clone()] },
        "utics": {"$in": [sector.clone()]}
    };

    // Load .env file into the environment
    dotenv().ok();

    // Reusable connection to MongoDB
    let mongo_uri = env::var("MONGODB_URI").expect("Environment variable MONGODB_URI not set");
    let mongo_client = Client::with_uri_str(&mongo_uri).await.expect("Failed to initialize MongoDB client");

    let df = _uparams_df(mongo_client, query).await.expect("Failed to fetch data");
    print!("{:#?}", df);

    // convert DataFrame to JSON. Use Polars write to json into memory buffer. Return as a body

    // Return the JSON as a response
    return StatusCode::OK
}


/* does this need to be json */
async fn send_it() -> impl IntoResponse {

     // JSON input as a string. normally passed via post command
     let inputs = r#"{"company_name": "a", "primary_sector": "bb", "primary_country": "c", "total_revenue": "3"}"#;

     // Parse JSON input to `serde_json::Value`
     let json_data: Value = serde_json::from_str(inputs).expect("Failed to parse JSON");
 
     // Convert to Polars DataFrame
     let df = JsonReader::new(Cursor::new(json_data.to_string()))
         .with_json_format(JsonFormat::Json)
         .finish()
         .expect("Failed to create DataFrame");

    print!{"{:?}", df}

    return StatusCode::OK
}


#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {

    // example query http://127.0.0.1:8000/api/GBR/UT201050
    // let router = Router::new().route("/api/:country/:sector", get(get_mongodb));
    let router = Router::new().route("/send_it/", get(send_it));

    Ok(router.into())
}