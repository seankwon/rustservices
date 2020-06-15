use serde::{Deserialize, Serialize};
use actix_web::{Result, web, HttpResponse};
use crate::db;
use crate::users::model;

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
 * [x] - validate user and password
 * [x] - get user from db
 * [] - create session token within state
 * [x] - create and return token
*/

pub async fn login(json: web::Json<Login>) -> Result<HttpResponse, HttpResponse> {
    if json.username == "seankwon" {
        return Err(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let token = model::create_token(&json.username).unwrap();
    Ok(HttpResponse::Ok().json(LoginResponse { token: token }))
}

pub async fn create(json: web::Json<model::NewUser>) -> HttpResponse {
    // TODO: needs specific validation
    let conn = db::establish_connection();
    match model::create_user(&conn, &json) {
        Ok(_) => HttpResponse::Ok().body("Ok!"),
        Err(_) => HttpResponse::InternalServerError().body("did not work")
    }
}

