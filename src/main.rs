use axum::{
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use std::env;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load environment variables");

    let port = env::var("PORT").unwrap_or(String::from("3000"));
    let db_config = env::var("DB_CONFIG").expect("Postgres settings aren't in the environment");

    let (client, connection) = tokio_postgres::connect(&db_config, NoTls)
        .await
        .expect("Failed to connect to postgres");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {e}");
        }
    });

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
