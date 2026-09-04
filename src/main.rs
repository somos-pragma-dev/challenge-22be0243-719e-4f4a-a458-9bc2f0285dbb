use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod domain;
mod application;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
dotenv().ok();

let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
infrastructure::establish_connection(&database_url);

HttpServer::new(|| {
    App::new()
       .configure(application::config)
})
.bind("127.0.0.1:8080")?
.run()
.await
}