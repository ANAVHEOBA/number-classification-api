use actix_web::{web, HttpResponse, Result};
use crate::models::{NumberQuery, NumberResponse, ErrorResponse};
use crate::services::number_service::NumberService;

pub async fn classify_number(query: web::Query<NumberQuery>, service: web::Data<NumberService>) -> Result<HttpResponse> {
    // Parse the number
    let number = match query.number.parse::<i64>() {
        Ok(n) => {
            if n < 0 || n > 1_000_000 {
                return Ok(HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json(ErrorResponse {
                        number: query.number.clone(),
                        error: true,
                        message: "Number must be between 0 and 1,000,000".to_string(),
                    }));
            }
            n
        },
        Err(_) => {
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .json(ErrorResponse {
                    number: query.number.clone(),
                    error: true,
                    message: "Invalid number format".to_string(),
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
        success: true,
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(response))
}