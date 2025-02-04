use actix_web::{web, HttpResponse, Result};
use crate::models::NumberQuery;
use crate::services::number_service::NumberService;
use serde_json::json;

pub async fn classify_number(
    query: web::Query<NumberQuery>,
    service: web::Data<NumberService>,
) -> Result<HttpResponse> {
    // Parse the number
    let number = match query.number.parse::<i64>() {
        Ok(n) => {
            if n.abs() > 1_000_000 {
                return Ok(HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json(json!({
                        "number": query.number,
                        "error": true
                    })));
            }
            n
        }
        Err(_) => {
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
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

    // Create response with exact formatting
    let response_text = format!(
        "{{\n    \"number\": {},\n    \"is_prime\": {},\n    \"is_perfect\": {},\n    \"properties\": [\"{}\", \"{}\"],\n    \"digit_sum\": {},\n    \"fun_fact\": \"{}\"\n}}",
        number,
        service.is_prime(number.abs()),
        service.is_perfect(number.abs()),
        properties[0],
        properties[1],
        service.digit_sum(number.abs()),
        fun_fact
    );

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(response_text))
}