use axum::{
    response::IntoResponse, routing::{post, get}, Json, Router, http::StatusCode
};

use shuttle_axum::ShuttleAxum;

use polars::prelude::*;
use polars_io::SerReader;
use polars_io::prelude::{JsonReader, JsonFormat};

use std::io::Cursor;
use serde_json::{Value, json};

/* mongodb_to_polars imports */
use std::env;
use dotenvy::dotenv;
use mongodb::{bson::{doc, Document}, Client, Cursor as MCursor};
mod models{
    pub mod spectacle;
}
use models::spectacle::UParams;
// use StreamExt from futures to asynchronously collect the cursor items.
use futures::StreamExt;


async fn mongodb_to_polars() -> impl IntoResponse {
    dotenv().ok(); // load environment variables

    // Connect to MongoDB
    let mongo_uri = env::var("MONGODB_URI").expect("Environment variable MONGODB_URI not set");
    let mongo_client = Client::with_uri_str(&mongo_uri).await.expect("Failed to initialize MongoDB client");

    // Construct the query document for MongoDB
    let data = vec!["USA", "CAN"];
 
    let query_filter: Document = doc! {
        "iso3": { "$in": data }
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

    // Cursor<T> is a query stream != Json
    // https://docs.rs/mongodb/3.1.0/mongodb/struct.Cursor.html

    /* TERRIBLE CODE STARTS HERE. MongoDB to Polars */
    // Collect the documents into a JSON array
    let json_array: Value = serde_json::json!(cursor
        .map(|doc_result| {
            doc_result
                .map(|doc| serde_json::to_value(doc).expect("Failed to convert to JSON"))
                .unwrap_or_else(|_| json!({ "error": "Failed to fetch document" }))
        })
        .collect::<Vec<_>>()
        .await);


    let df = JsonReader::new(Cursor::new(json_array.to_string()))
        .with_json_format(JsonFormat::Json)
        .finish();

    print!("{:?}", df);

    return StatusCode::OK
}

async fn clean_inputs(Json(inputs): Json<Value>) -> impl IntoResponse {
     // Convert JSON to Polars DataFrame
     let df = JsonReader::new(Cursor::new(inputs.to_string()))
         .with_json_format(JsonFormat::Json)
         .finish()
         .expect("Failed to create DataFrame");

    // Convert to LazyFrame to perform expression-based transformations
    let lf = df.lazy();
    
    // Multiply proportions (90%) by total_revenue ($)
    let lf = lf.with_columns(vec![
        when(col("metric").eq(lit("revenue_proportion")))
            .then(col("total_revenue") * col("value") * lit(0.01))
            .otherwise(lit(0))
            .alias("absolute_proportion")
    ]);

    // Filter to use Inner Join with uParams
    let lf = lf.filter(col("metric").eq(lit("revenue_proportion")));

    // Create a list from the DataFrame. This cannot be done with a LazyFrame,
    // as the data is only materialized when .collect() is called.
    // lf.clone() is important here
    let cloned_df = lf.clone().collect().expect("Failed to collect the LazyFrame into a DataFrame.");
    // select a single column
    let search_term = cloned_df.column("search_term");

    // Use `while let` to loop through the values
    while let Some(value) = search_term.iter().next() {
        println!("{:?}", value);
    }


    /*
    1. Use list of search_terms to fitler monogDB query
    2. inner join filtered uparams to clean_inputs
    */

    // This code is only for printing
    let lf = lf.select([
        col("pkey"),
        col("search_term"),
        col("absolute_proportion")
    ]);

    let df = lf.collect().expect("Failed to collect the LazyFrame into a DataFrame.");
    println!("{:?}", df);

    return StatusCode::OK
}


#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    /* POST using flattened structure =
    curl http://127.0.0.1:8000/clean_inputs/ \
    -H "Content-Type: application/json" \
    -d '[
    {"pkey":9158,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Sugar farm","metric":"lon","value":-51.107786,"unit":null,"in_portfolio":false,"external_id":"5da66e19-4f47-47a0-a7d8-c35ba7b1764c","secondary_search_term":"agricultural_farm"},
    {"pkey":9159,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Manufacturing centre","metric":"lon","value":-46.956677,"unit":null,"in_portfolio":false,"external_id":"dbcf7da6-79e2-4e29-95fe-9fdd11855bf8","secondary_search_term":"manufacturing_plant"},
    {"pkey":9160,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Distribution centre","metric":"lon","value":-54.865848,"unit":null,"in_portfolio":false,"external_id":"4120c73c-7d9a-400f-9054-5aeeb3cc81d2","secondary_search_term":"warehouse_storage"},
    {"pkey":9161,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Dairy farm 2","metric":"ownership","value":0,"unit":null,"in_portfolio":false,"external_id":"c85e073c-6450-44bb-8558-8baf44fddc95","secondary_search_term":"agricultural_farm"},
    {"pkey":9162,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Sugar farm","metric":"ownership","value":0,"unit":null,"in_portfolio":false,"external_id":"5da66e19-4f47-47a0-a7d8-c35ba7b1764c","secondary_search_term":"agricultural_farm"},
    {"pkey":9139,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":2023,"category":"financials","search_term":null,"metric":"sga","value":210,"unit":null,"in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9140,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":2023,"category":"financials","search_term":null,"metric":"cogs","value":106,"unit":null,"in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9141,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"alignment_statement","search_term":null,"metric":"board_oversight","value":1,"unit":null,"in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9142,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"alignment_statement","search_term":null,"metric":"annual_ghg_disclosure","value":1,"unit":null,"in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9143,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"revenue_by_sector","search_term":"UT30202030AA","metric":"revenue_proportion","value":100,"unit":"primary","in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9144,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"revenue_by_country","search_term":"BRA","metric":"revenue_proportion","value":90,"unit":"primary","in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9145,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"revenue_by_country","search_term":"ARG","metric":"revenue_proportion","value":7,"unit":null,"in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9146,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"revenue_by_country","search_term":"CHL","metric":"revenue_proportion","value":3,"unit":null,"in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9147,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"costs","search_term":"dairy","metric":"initial_value","value":30.9,"unit":null,"in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9148,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"costs","search_term":"sugar","metric":"initial_value","value":19.8,"unit":null,"in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9149,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":2023,"category":"emissions","search_term":null,"metric":"current_emissions","value":103,"unit":"kg","in_portfolio":false,"external_id":null,"secondary_search_term":null},
    {"pkey":9152,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Dairy farm 1","metric":"ownership","value":0,"unit":"primary","in_portfolio":false,"external_id":"7455acdc-d559-4d19-89c2-446c02fdeb32","secondary_search_term":"agricultural_farm"},
    {"pkey":9153,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Dairy farm 2","metric":"lat","value":-40.691247,"unit":null,"in_portfolio":false,"external_id":"c85e073c-6450-44bb-8558-8baf44fddc95","secondary_search_term":"agricultural_farm"},
    {"pkey":9154,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Sugar farm","metric":"lat","value":-28.261421,"unit":null,"in_portfolio":false,"external_id":"5da66e19-4f47-47a0-a7d8-c35ba7b1764c","secondary_search_term":"agricultural_farm"},
    {"pkey":9155,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Manufacturing centre","metric":"lat","value":-23.77482,"unit":null,"in_portfolio":false,"external_id":"dbcf7da6-79e2-4e29-95fe-9fdd11855bf8","secondary_search_term":"manufacturing_plant"},
    {"pkey":9156,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Distribution centre","metric":"lat","value":-25.373218,"unit":null,"in_portfolio":false,"external_id":"4120c73c-7d9a-400f-9054-5aeeb3cc81d2","secondary_search_term":"warehouse_storage"},
    {"pkey":9157,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Dairy farm 2","metric":"lon","value":-62.649375,"unit":null,"in_portfolio":false,"external_id":"c85e073c-6450-44bb-8558-8baf44fddc95","secondary_search_term":"agricultural_farm"},
    {"pkey":9163,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Manufacturing centre","metric":"ownership","value":0,"unit":null,"in_portfolio":false,"external_id":"dbcf7da6-79e2-4e29-95fe-9fdd11855bf8","secondary_search_term":"manufacturing_plant"},
    {"pkey":9164,"total_revenue":999,"org":"natasha","three_random_word_id":"actual-goes-yourself","company_name":"Ice Creamapalooza","year":null,"category":"asset","search_term":"Distribution centre","metric":"ownership","value":0,"unit":null,"in_portfolio":false,"external_id":"4120c73c-7d9a-400f-9054-5aeeb3cc81d2","secondary_search_term":"warehouse_storage"}
    ]'
    */
    /* GET = http://127.0.0.1:8000/mongodb_to_polars/ */

    let router = Router::new()
        .route("/clean_inputs/", post(clean_inputs))
        .route("/mongodb_to_polars/", get(mongodb_to_polars))
    ;

    Ok(router.into())
}
