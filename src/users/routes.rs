use serde::{Deserialize, Serialize};
use actix_web::{Result, web, HttpResponse};
use crate::db;
use crate::users::model;
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

/*
 * [] - create sessions token
*/
pub async fn login(json: web::Json<Login>) -> Result<HttpResponse, HttpResponse> {
    use crate::schema::users::dsl::*;
    let conn = db::establish_connection();
    let query: model::User = users
        .filter(username.eq(&json.username))
        .first(&conn)
        .expect("unexpected");
    let token = model::create_token(&json.username).unwrap();
    // Ok(HttpResponse::Ok().json(LoginResponse { token: token }))
    Ok(HttpResponse::Ok().json(query))
}

pub async fn create(json: web::Json<model::NewUser>) -> HttpResponse {
    // TODO: needs specific validation
    let conn = db::establish_connection();
    match model::create_user(&conn, &json) {
        Ok(_) => HttpResponse::Ok().body("Ok!"),
        Err(_) => HttpResponse::InternalServerError().body("did not work")
    }
}
