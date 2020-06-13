#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod users;
mod schema;
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on port 8088");
    let conn = db::establish_connection();
    let username = String::from("seankwon");
    let password = String::from("foo");
    let email = String::from("kwon@sean.com");
    users::create_user(&conn, &username, &password, &email);
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(users::login))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
