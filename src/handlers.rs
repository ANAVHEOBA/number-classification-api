use actix_web::{web, HttpResponse, Result};
use crate::models::{NumberQuery, NumberResponse, ErrorResponse};
use crate::services::number_service::NumberService;
use serde_json::{json, to_string_pretty};
use indexmap::IndexMap;

pub async fn classify_number(
    query: web::Query<NumberQuery>,
    service: web::Data<NumberService>,
) -> Result<HttpResponse> {
    // Parse the number
    let number = match query.number.parse::<i64>() {
        Ok(n) => {
            if n.abs() > 1_000_000 {
                let mut map = IndexMap::new();
                map.insert("number", json!(query.number.clone()));
                map.insert("error", json!(true));
                return Ok(HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(to_string_pretty(&map)?));
            }
            n
        }
        Err(_) => {
            let mut map = IndexMap::new();
            map.insert("number", json!(query.number.clone()));
            map.insert("error", json!(true));
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(to_string_pretty(&map)?));
        }
    };

    let response = NumberResponse {
        number,
        is_prime: service.is_prime(number.abs()),
        is_perfect: service.is_perfect(number.abs()),
        properties: service.get_properties(number),
        digit_sum: service.digit_sum(number.abs()),
        fun_fact: match service.get_fun_fact(number).await {
            Ok(fact) => fact,
            Err(_) => {
                if service.is_armstrong(number.abs()) {
                    let digits: Vec<i64> = number.abs().to_string()
                        .chars()
                        .map(|c| c.to_digit(10).unwrap() as i64)
                        .collect();
                    let power = digits.len() as u32;
                    format!("{} is an Armstrong number because {}^{} + {}^{} + {}^{} = {}",
                        number.abs(),
                        digits[0], power,
                        digits[1], power,
                        digits[2], power,
                        number.abs()
                    )
                } else {
                    format!("{} is the number you provided", number)
                }
            }
        },
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(to_string_pretty(&response)?))
}