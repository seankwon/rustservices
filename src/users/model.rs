use crate::schema::{users, sessions};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use jsonwebtoken::errors::{Error};
use jsonwebtoken::{TokenData, decode, encode, DecodingKey, EncodingKey, Header, Validation};
use bcrypt::{DEFAULT_COST, hash};
use chrono::{Local, NaiveDateTime}; // This type is used for date field in Diesel.
use serde::{Deserialize, Serialize};
use nanoid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, PartialEq, Identifiable, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    username: String,
    email: String,
    #[serde(skip)]
    id: String,
    created_at: NaiveDateTime,
    #[serde(skip)]
    password: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
}

#[derive(Debug, PartialEq, Queryable, Insertable, Serialize, Deserialize, Associations)]
#[belongs_to(User)]
#[table_name = "sessions"]
pub struct Session {
    #[serde(skip)]
    id: String,
    username: String,
    secret: String,
    created_at: NaiveDateTime,
    user_id: Option<String>,
}


pub fn create_user(conn: &PgConnection, user: &NewUser) -> Result<usize, diesel::result::Error> {
    let new_user = User { 
        username: user.username.clone(),
        email: user.email.clone(),
        password: hash(user.password.clone(), DEFAULT_COST).unwrap().clone(),
        id: nanoid::simple(),
        created_at: Local::now().naive_local(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
}

pub fn create_token(username: &String) -> Result<String, Error> {
    // TODO: store secret token somewhere
    let key = b"SECRET_TOKEN";
    let my_claim = Claims { 
        sub: username.clone(), 
        exp: 10000000000
    };
    encode(&Header::default(), &my_claim, &EncodingKey::from_secret(key))
}

pub fn create_session(conn: &PgConnection, user: &User) -> Result<Session, diesel::result::Error> {
    let new_session = Session { 
        username: user.username.clone(),
        id: nanoid::simple(),
        user_id: Some(user.id.clone()),
        created_at: Local::now().naive_local(),
        secret: nanoid::simple(),
    };

    diesel::insert_into(sessions::table)
        .values(&new_session)
        .get_result(conn)
}

pub fn validate_token(token: &String, username: &String) -> Result<TokenData<Claims>, Error> {
    let key = b"SECRET_TOKEN";
    let validation = Validation { sub: Some(username.clone()), ..Validation::default() };
    decode::<Claims>(token, &DecodingKey::from_secret(key), &validation)
}
