#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate nanoid;

mod db;
mod users;
mod schema;
use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on port 8088");
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(users::routes::login))
            .route("/users", web::post().to(users::routes::create))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
