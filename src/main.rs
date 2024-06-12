use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct QueryParams {
    t: Option<String>,
}

async fn handler(Query(params): Query<QueryParams>) -> impl IntoResponse {
    (StatusCode::OK, format!("t = {:?}", params.t))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/pessoas", post(|| async { "Post" }))
        .route("/pessoas/:id", get(|| async { "Get Id" }))
        .route("/pessoas", get(handler))
        .route("/contagem-pessoas", get(|| async { "Contagem Pessoas" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
