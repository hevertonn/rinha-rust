use axum::{
    extract::{Path, Query},
    http::StatusCode,
};
use std::collections::HashMap;

pub async fn feth_person_by_id(Path(user_id): Path<String>) -> String {
    user_id
}

pub async fn fetch_people_by_query(
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, String) {
    match params.get("t") {
        Some(search_term) => (StatusCode::OK, search_term.clone()),
        None => (StatusCode::BAD_REQUEST, "Search term not found".to_string()),
    }
}
