// src/handlers.rs

use actix_web::{web, HttpResponse, Result};
use serde_json::json;

use crate::models::{ErrorResponse, NumberQuery, NumberResponse};
use crate::services::number_service::NumberService;

pub async fn classify_number(
    query: web::Query<NumberQuery>,
    service: web::Data<NumberService>,
) -> Result<HttpResponse> {
    // Parse the number
    let number = match query.number.parse::<i64>() {
        Ok(n) => {
            if n.abs() > 1_000_000 {
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    number: query.number.clone(),
                    error: true,
                }));
            }
            n
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                number: query.number.clone(),
                error: true,
            }));
        }
    };

    // Retrieve properties
    let properties = service.get_properties(number);

    // Get fun fact
    let fun_fact = match service.get_fun_fact(number).await {
        Ok(fact) => fact,
        Err(_) => {
            if service.is_armstrong(number.abs()) {
                let digits: Vec<char> = number.abs().to_string().chars().collect();
                if digits.len() == 3 {
                    format!(
                        "{} is an Armstrong number because {}^3 + {}^3 + {}^3 = {}",
                        number,
                        digits[0],
                        digits[1],
                        digits[2],
                        number.abs()
                    )
                } else {
                    format!("{} is an Armstrong number.", number)
                }
            } else {
                format!("{} is the number you provided.", number)
            }
        }
    };

    // Construct the response
    let response = NumberResponse {
        number,
        is_prime: service.is_prime(number.abs()),
        is_perfect: service.is_perfect(number.abs()),
        properties,
        digit_sum: service.digit_sum(number.abs()),
        fun_fact,
    };

    Ok(HttpResponse::Ok().json(response))
}