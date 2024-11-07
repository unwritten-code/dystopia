use axum::{response::IntoResponse, routing::get, Router, Json};
use shuttle_axum::ShuttleAxum;
use mongodb::{bson::{doc, Document}, Client, Collection, Cursor as MongoCursor}; // Alias mongodb::Cursor to MongoCursor

use std::env;
use dotenvy::dotenv;
use serde::Serialize;

mod models{
    pub mod spectacle;
}

use models::spectacle::UParams;

// End 
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: Option<String>,
}

async fn load_uparams(
        mongo_client: Client,
        mongo_query: Document,
    ) -> mongodb::error::Result<Vec<String>>  {
  
    // store output in an array
    let mut all_docs = Vec::new();

    // select uparams collection from database
    let uparams: Collection<UParams> = mongo_client
        .database("delphi-dev")
        .collection("uparams");

    let mut cursor: MongoCursor<UParams> = uparams.find(mongo_query).limit(3).await?;

    while cursor.advance().await? {
        let doc = cursor.deserialize_current()?;
        all_docs.push(doc.iso3);
    };

    Ok(all_docs)
}


async fn my_get() -> impl IntoResponse {
    // Load .env file into the environment
    dotenv().ok();
    
    // MongoDB connection
    let mongo_uri = env::var("MONGODB_URI").expect("Environment variable MONGODB_URI not set");
    let mongo_client = Client::with_uri_str(&mongo_uri).await.expect("Failed to initialize MongoDB client");
    
    let mongo_query: Document = doc! {
      "iso3": { "$in": ["GBR", "AUS"] }
    };
    
     // Attempt to perform the query and handle potential errors
     match load_uparams(mongo_client, mongo_query).await {
        Ok(response) => {
            // If successful, wrap the data in ApiResponse
            Json(ApiResponse {
                success: true,
                data: Some(response),
                message: None,
            })
        }
        Err(e) => {
            // If there's an error, return a JSON error message
            Json(ApiResponse {
                success: false,
                data: None,
                message: Some(format!("Failed to load data: {}", e)),
            })
        }
    }
}





#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    
    // make a post that does the mongodb thing
    let router = Router::new().route("/mdb/", get(my_get));
    
    Ok(router.into())
}