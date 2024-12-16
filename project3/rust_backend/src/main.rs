use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening On Port: 3000");

    axum::serve(listener, app).await.unwrap();
}

async fn get_handler() -> &'static str {
    "Hello World This is First step on learning Rust!"
}
