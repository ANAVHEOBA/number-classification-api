use actix_web::{web, HttpResponse, Result};
use crate::models::{NumberQuery, NumberResponse, ErrorResponse};
use crate::services::number_service::NumberService;

pub async fn classify_number(
    query: web::Query<NumberQuery>,
    service: web::Data<NumberService>,
) -> Result<HttpResponse> {
    log::info!("Received request for number: {}", query.number);
    
    // Parse the number
    let number = match query.number.parse::<i64>() {
        Ok(n) => {
            // Remove the negative number check, only keep the upper limit
            if n > 1_000_000 {
                log::warn!("Number {} out of valid range", n);
                return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                    number: query.number.clone(),
                    error: true,
                }));
            }
            n
        }
        Err(_) => {
            log::warn!("Failed to parse number {}", query.number);
            return Ok(HttpResponse::BadRequest().json(ErrorResponse {
                number: query.number.clone(),
                error: true,
            }));
        }
    };

    // Get number properties
    let is_prime = service.is_prime(number);
    let is_perfect = service.is_perfect(number);
    let properties = service.get_properties(number);
    let digit_sum = service.digit_sum(number);
    
    // Get fun fact
    let fun_fact = match service.get_fun_fact(number).await {
        Ok(fact) => fact,
        Err(_) => format!("{} is the number you provided", number),
    };

    // Create response
    let response = NumberResponse {
        number,
        is_prime,
        is_perfect,
        properties,
        digit_sum,
        fun_fact,
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(response))
}