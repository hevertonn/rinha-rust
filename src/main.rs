use axum::{
    routing::{get, post},
    Router,
};

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load environment variables!");

    let port = env::var("PORT").unwrap_or(String::from("8080"));

    let router: Router = Router::new()
        .route("/pessoas", post("post pessoas"))
        .route("/pessoas/", get("get pessoas id"))
        .route("/pessoas", get("get pessoas query"))
        .route("/contagem-pessoas", get("get contagem-pessoas"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}
