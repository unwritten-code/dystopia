use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct NatureBySector{
    pub nature_risk: String,
    pub utics: String,
    pub value: String,
    pub materiality: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UParams{
    pub iso3: String,
    pub scenario: String,
    pub utics: String,
    pub year: i32,
    pub value: f64,
    pub delphi_financial_var: String,
}