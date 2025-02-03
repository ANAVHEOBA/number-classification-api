use actix_web::{web, HttpResponse, Result};
use crate::models::{NumberQuery, NumberResponse, ErrorResponse};
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

    let is_prime = service.is_prime(number.abs());
    let is_perfect = service.is_perfect(number.abs());
    let properties = service.get_properties(number);
    let digit_sum = service.digit_sum(number.abs());
    let fun_fact = service.get_fun_fact(number).await?;

    Ok(HttpResponse::Ok().json(NumberResponse {
        number,
        is_prime,
        is_perfect,
        properties,
        digit_sum,
        fun_fact,
    }))
}