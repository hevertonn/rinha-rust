use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let router: Router = Router::new().route("/", get("Hello world!"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
