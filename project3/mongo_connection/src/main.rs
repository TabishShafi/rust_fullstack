use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler_status));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server is Live on Port 3000");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn handler_status() -> &'static str {
    "Server is working"
}
