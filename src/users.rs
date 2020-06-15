// use crate::db;
use diesel::prelude::*;
use crate::schema::users;
use bcrypt::{DEFAULT_COST, hash};
use chrono::{Local, NaiveDateTime}; // This type is used for date field in Diesel.
use jsonwebtoken::errors::{ErrorKind, Error};
use jsonwebtoken::{TokenData, decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use actix_web::{Result, web, HttpResponse};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    username: String,
    email: String,
    #[serde(skip)]
    password: String,
    created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

/*
 * [x] - validate user and password
 * [] - get user from db
 * [] - create session token within state
 * [x] - create and return token
*/

pub async fn login(json: web::Json<Login>) -> Result<HttpResponse, HttpResponse> {
    if json.username == "seankwon" {
        return Err(HttpResponse::Unauthorized().body("Unauthorized"));
    }
    let token = create_token(&json.username).unwrap();
    Ok(HttpResponse::Ok().json(LoginResponse { token: token }))
}

pub fn create_user(conn: &SqliteConnection, user: &NewUser) -> Result<usize, diesel::result::Error> {
    // optimize this with borrowed vals
    let new_user = User { 
        username: user.username.clone(),
        email: user.email.clone(),
        password: hash(user.password.clone(), DEFAULT_COST).unwrap().clone(),
        created_at: Local::now().naive_local(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}

fn create_token(username: &String) -> Result<String, Error> {
    let key = b"SECRET_TOKEN";
    let my_claim = Claims { 
        sub: username.clone(), 
        exp: 10000000000
    };
    encode(&Header::default(), &my_claim, &EncodingKey::from_secret(key))
}

fn validate_token(token: &String, username: &String) -> Result<TokenData<Claims>, Error> {
    let key = b"SECRET_TOKEN";
    let validation = Validation { sub: Some(username.clone()), ..Validation::default() };
    decode::<Claims>(token, &DecodingKey::from_secret(key), &validation)
}

fn main () {
    let username = String::from("sean@kwon");
    let token = match create_token(&username) {
        Ok(t) => t,
        Err(_) => panic!(),
    };
    let token_data = match validate_token(&token, &username) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"), // Example on how to handle a specific error
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"), // Example on how to handle a specific error
            _ => panic!("{}", err),
        },
    };
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
}
