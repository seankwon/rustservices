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
    /*
    let conn = db::establish_connection();
    let user = users::NewUser {
        email: String::from("seankwon@foo.com"),
        password: String::from("fdsaf"),
        username: String::from("subaru"),
    };
    users::create_user(&conn, &user);
    */
    HttpServer::new(|| {
        App::new()
            .route("/login", web::post().to(users::login))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
