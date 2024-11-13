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

use libsql::Database;
// use shuttle_runtime::{Secret, SecretStore};


async fn clean_inputs(Json(inputs): Json<Value>) -> impl IntoResponse {
    // Convert JSON data to a string and wrap it in a Cursor
    let json_data = serde_json::to_string(&inputs).expect("Failed to serialize JSON");

     // Convert JSON to Polars DataFrame
     let df = JsonReader::new(Cursor::new(json_data))
         .with_json_format(JsonFormat::Json)
         .finish()
         .expect("Failed to create DataFrame");

    // Convert to LazyFrame to perform expression-based transformations
    let df = df.lazy();
    
    // multiply proportions (90%) by total_revenue ($)
    let df = df.with_columns(vec![
        when(col("metric").eq(lit("revenue_proportion")))
            .then(col("total_revenue") * col("value") * lit(0.01))
            .otherwise(lit(0))
            .alias("absolute_proportion")
    ]);

    // turn list of countries into a search param for
    let df = df.filter(col("metric").eq(lit("revenue_proportion")));

    //for printing
    let df = df.select([
        col("pkey"),
        col("search_term"),
        col("absolute_proportion")
    ]);

    // inner join

    // Execute the lazy operation
    let df = df.collect();

    print!{"{:?}", df}

    // save to sql

    return StatusCode::OK
}


#[shuttle_runtime::main]
async fn main(
    #[shuttle_turso::Turso(
        addr = "libsql://test-ayephillip.turso.io",
        local_addr ="libsql://test-ayephillip.turso.io",
        token = ""
    )] db: Database,
) -> ShuttleAxum {

    let conn = db.connect().expect("msg");

    let mut rows = conn.query("SELECT * FROM Students", ())
        .await
        .expect("Failed to execute query");

   while let Some(row) =  rows.next().await.expect("msg") {
        println!("Table name: {:?}", row);   
    } 

    // useful query = "SELECT name FROM sqlite_master WHERE type='table'"
    // https://docs.turso.tech/sdk/rust/quickstart#remote-only
    // https://github.com/tursodatabase/libsql/tree/main/libsql/examples
    //https://docs.turso.tech/sdk/rust/guides/axum



    /*
    curl http://127.0.0.1:8000/clean_inputs/ \
    -H "Content-Type: application/json" \
    -d '[
         {
        "pkey": 9151,
        "metric": "revenue_proportion",
        "three_random_word_id": "actual-goes-yourself",
        "total_revenue": 89,
        "value": 99
    }
    ]'
    */

    /* Flattened structure
    curl http://127.0.0.1:8010/clean_inputs/ \
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
    let router = Router::new().route("/clean_inputs/", post(clean_inputs));


    Ok(router.into())
}
