mod handlers; // Add this line to declare the handlers module
mod db;       // Declare the db module if not done already
mod models;   // Declare the models module

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use crate::handlers::{create_patient, get_patients};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env variables

    HttpServer::new(|| {
        App::new()
            .route("/patients", web::post().to(create_patient))
            .route("/patients", web::get().to(get_patients))
    })
    .bind("127.0.0.1:8080")? // Bind server to localhost at port 8080
    .run()
    .await
}
