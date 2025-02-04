use actix_web::{web, HttpResponse, Result};
use crate::models::{NumberQuery, NumberResponse, ErrorResponse};
use crate::services::number_service::NumberService;
use serde_json::{to_string_pretty, json};

pub async fn classify_number(
    query: web::Query<NumberQuery>,
    service: web::Data<NumberService>,
) -> Result<HttpResponse> {
    // Parse the number
    let number = match query.number.parse::<i64>() {
        Ok(n) => {
            if n.abs() > 1_000_000 {
                let error_json = json!({
                    "number": query.number.clone(),
                    "error": true
                });
                return Ok(HttpResponse::BadRequest()
                    .insert_header(("Content-Type", "application/json"))
                    .body(to_string_pretty(&error_json)?));
            }
            n
        }
        Err(_) => {
            let error_json = json!({
                "number": query.number.clone(),
                "error": true
            });
            return Ok(HttpResponse::BadRequest()
                .insert_header(("Content-Type", "application/json"))
                .body(to_string_pretty(&error_json)?));
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

    // Format response with proper indentation
    let json_str = to_string_pretty(&response)?;
    
    Ok(HttpResponse::Ok()
        .insert_header(("Content-Type", "application/json"))
        .body(json_str))
}