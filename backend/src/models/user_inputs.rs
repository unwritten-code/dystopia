use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Fundamentals {
    pub company_name: String,
    pub primary_sector: String,
    pub primary_country: String,
    pub total_revenue: String,
}
