use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let router: Router = Router::new()
        .route("/pessoas", post("post pessoas"))
        .route("/pessoas/", get("get pessoas id"))
        .route("/pessoas", get("get pessoas query"))
        .route("/contagem-pessoas", get("get contagem-pessoas"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
