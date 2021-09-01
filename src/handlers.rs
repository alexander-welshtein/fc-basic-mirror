use actix_web::{Error, HttpResponse, web};
use async_graphql::Request;

use crate::schemas::root::Schema;

pub async fn graphql(
    schema: web::Data<Schema>,
    request: web::Json<Request>,
) -> Result<HttpResponse, Error> {
    let response = schema.execute(request.into_inner()).await;
    let body = match serde_json::to_string(&response) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[E] JSON :: {}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
    )
}