use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NumberQuery {
    pub number: String,
}

#[derive(Serialize)]
pub struct NumberResponse {
    pub success: bool,
    pub number: i64,
    pub is_prime: bool,
    pub is_perfect: bool,
    pub properties: Vec<String>,
    pub digit_sum: i64,
    pub fun_fact: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: bool,
    pub number: String,
    pub message: String,
}