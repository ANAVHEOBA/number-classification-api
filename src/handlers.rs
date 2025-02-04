use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::models::NumberQuery;
use crate::services::number_service::NumberService;

pub async fn classify_number(
    query: web::Query<NumberQuery>,
    service: web::Data<NumberService>,
) -> Result<HttpResponse> {
    // Parse the number
    let number = match query.number.parse::<i64>() {
        Ok(n) => {
            if n.abs() > 1_000_000 {
                return Ok(HttpResponse::BadRequest()
                    .json(json!({
                        "number": query.number,
                        "error": true
                    })));
            }
            n
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest()
                .json(json!({
                    "number": query.number,
                    "error": true
                })));
        }
    };

    let properties = service.get_properties(number);
    let fun_fact = match service.get_fun_fact(number).await {
        Ok(fact) => fact,
        Err(_) => {
            if service.is_armstrong(number.abs()) {
                format!("{} is an Armstrong number because {}^3 + {}^3 + {}^3 = {}",
                    number.abs(),
                    number.to_string().chars().nth(0).unwrap(),
                    number.to_string().chars().nth(1).unwrap(),
                    number.to_string().chars().nth(2).unwrap(),
                    number.abs()
                )
            } else {
                format!("{} is the number you provided", number)
            }
        }
    };

    // Use serde_json::json! macro to create proper JSON response
    let response = json!({
        "number": number,
        "is_prime": service.is_prime(number.abs()),
        "is_perfect": service.is_perfect(number.abs()),
        "properties": properties,
        "digit_sum": service.digit_sum(number.abs()),
        "fun_fact": fun_fact
    });

    Ok(HttpResponse::Ok().json(response))
}