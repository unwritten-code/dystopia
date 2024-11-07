use axum::{
    body::Body, http::StatusCode, response::IntoResponse, routing::post, Json, Router
};

//https://www.mongodb.com/docs/drivers/rust/current/usage-examples/findOne/#std-label-rust-find-one-usage
use mongodb::{bson::{doc, Document}, Client, Collection, Cursor as MongoCursor}; // Alias mongodb::Cursor to MongoCursor


use serde::{Deserialize, Serialize};
use std::{env, io::Cursor};
use dotenvy::dotenv;

// used to create static files / webpage
use tower_http::services::ServeDir;






async fn _hello_word(Json(json): Json<PostParams>) -> String {
    format!("POST returns key: {0}, value: {1}", json.company_name, json.total_revenue)
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
      // load .env file into the environment
      dotenv().ok();

    // mongodb connection
     let mongo_uri = env::var("MONGODB_URI").expect("Environment variable MONGODB_URI not set");
     let mongo_client = Client::with_uri_str(&mongo_uri).await.expect("Failed to initialize MongoDB client");

    let mongo_query: Document = doc! {
        "iso3": { "$in": ["GBR", "AUS"] }
    };
    let data = load_uparams(mongo_client, mongo_query).await;
    print!("{:?}", data);

    // create a static page for documentation
    let static_files = Router::new().nest_service("/", ServeDir::new("assets"));
    
    // routes
    let dynamic_route = Router::new().route("/api/", post(json2excel));
    let router = static_files.merge(dynamic_route);

    Ok(router.into())
}