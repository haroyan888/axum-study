use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::repositories::TodoRepository;

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id).await.or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository> (
    Extension(repository): Extension<Arc<T>>
) -> Result<impl IntoResponse, StatusCode> {
    let todos = repository.all().await.or(Err(StatusCode::))
    Ok((StatusCode::OK, Json(todos)))
}