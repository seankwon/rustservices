use actix_web::{Error, Responder, HttpRequest, HttpResponse};
use serde::{Serialize};
use futures::future::{ready, Ready};
use serde_json;

#[derive(Serialize)]
struct MyJSON {
    name: &'static str,
}

impl Responder for MyJSON {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        println!("well i ran");

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

pub async fn index() -> impl Responder {
    MyJSON { name: "sean kwon" }
}
