use serde::{Deserialize, Serialize};
use serde_json::{Value, json, Map};

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
        // Create ordered map to maintain field order
        let mut map = Map::new();
        map.insert("number".to_string(), json!(self.number));
        map.insert("is_prime".to_string(), json!(self.is_prime));
        map.insert("is_perfect".to_string(), json!(self.is_perfect));
        map.insert("properties".to_string(), json!(self.properties));
        map.insert("digit_sum".to_string(), json!(self.digit_sum));
        map.insert("fun_fact".to_string(), json!(self.fun_fact));

        Value::Object(map).serialize(serializer)
    }
}