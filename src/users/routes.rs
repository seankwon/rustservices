use serde::{Deserialize, Serialize};
use actix_web::{Result, web, HttpResponse};
use crate::db;
use crate::users::model;
use diesel::prelude::*;
use bcrypt::{verify};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

pub async fn login(json: web::Json<Login>) -> Result<HttpResponse, HttpResponse> {
    use crate::schema::users::dsl::*;
    let conn = db::establish_connection();
    let query: model::User = users
        .filter(username.eq(&json.username))
        .first(&conn)
        .map_err(|_| HttpResponse::NotFound().body("Not Found"))?;
    let verified = verify(&json.password, &query.password);

    match verified {
        Ok(_) => {
            model::create_session(&conn, &query).unwrap();
            Ok(HttpResponse::Ok().body("ok!"))
        }
        Err(e) => {
            println!("{}", e);
            Err(HttpResponse::Unauthorized().body("nope!"))
        }
    }
}

pub async fn create(json: web::Json<model::NewUser>) -> HttpResponse {
    // TODO: needs specific validation
    let conn = db::establish_connection();
    match model::create_user(&conn, &json) {
        Ok(_) => HttpResponse::Ok().body("Ok!"),
        Err(_) => HttpResponse::InternalServerError().body("did not work")
    }
}
