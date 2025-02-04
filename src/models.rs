use serde::Deserialize;

#[derive(Deserialize)]
pub struct NumberQuery {
    pub number: String,
}

pub struct NumberResponse {
    pub number: i64,
    pub is_prime: bool,
    pub is_perfect: bool,
    pub properties: Vec<String>,
    pub digit_sum: i64,
    pub fun_fact: String,
}