use axum::extract::Path;

pub async fn feth_person_by_id(Path(user_id): Path<String>) -> String {
    user_id
}
