use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/pessoas", post(|| async { "Post" }))
        .route("/pessoas/:id", get(|| async { "Get user by id" }))
        .route("/pessoas", get(|| async { "Get user by query" }))
        .route("/contagem-pessoas", get(|| async { "Contagem pessoas" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
