use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct NumberQuery {
    pub number: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub number: String,
    pub error: bool,
}

pub struct NumberResponse {
    pub number: i64,
    pub is_prime: bool,
    pub is_perfect: bool,
    pub properties: Vec<String>,
    pub digit_sum: i64,
    pub fun_fact: String,
}

impl Serialize for NumberResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = IndexMap::new();
        map.insert("number", json!(self.number));
        map.insert("is_prime", json!(self.is_prime));
        map.insert("is_perfect", json!(self.is_perfect));
        // Properly serialize properties as an array
        map.insert("properties", json!(self.properties));
        map.insert("digit_sum", json!(self.digit_sum));
        map.insert("fun_fact", json!(self.fun_fact));

        map.serialize(serializer)
    }
}