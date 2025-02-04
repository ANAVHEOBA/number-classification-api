use actix_web::{web, HttpResponse, Result};
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
                let error_response = "{\n    \"number\": \"".to_string() + &query.number + "\",\n    \"error\": true\n}";
                return Ok(HttpResponse::BadRequest()
                    .content_type("application/json")
                    .body(error_response));
            }
            n
        }
        Err(_) => {
            let error_response = "{\n    \"number\": \"".to_string() + &query.number + "\",\n    \"error\": true\n}";
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .body(error_response));
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

    // Create response with exact formatting and indentation
    let response = format!(
r#"{{
    "number": {},
    "is_prime": {},
    "is_perfect": {},
    "properties": ["{}","{}"],
    "digit_sum": {},
    "fun_fact": "{}"
}}"#,
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
        .body(response))
}