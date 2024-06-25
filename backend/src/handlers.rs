use std::sync::Arc;

use axum::{
    async_trait, extract::{Extension, FromRequest, Path, Request}, http::StatusCode, response::IntoResponse, BoxError, Json
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::repositories::{CreateTodo, RepositoryError, TodoRepository};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync + http_body::Body,
    S::Data: Send,
    S::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let message = format!("Json parse error: [{}]", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

pub async fn all_todo<T: TodoRepository> (
    Extension(repository): Extension<Arc<T>>
) -> Result<impl IntoResponse, StatusCode> {
    let res = repository.all().await;
    match res {
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(todos) =>Ok((StatusCode::OK, Json(todos)))
    }
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = repository.find(id).await;
    match res {
        Err(e) => match e {
            RepositoryError::NotFound(_) => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
        Ok(todo) => Ok((StatusCode::OK, Json(todo)))
    }
}

pub async fn create_todo<T: TodoRepository>(
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.create(payload).await.or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok((StatusCode::CREATED, Json(todo)))
}