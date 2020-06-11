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
