use std::sync::Arc;

use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path, Request},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::repositories::{CreateTodo, RepositoryError, TodoRepository, UpdateTodo};

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    // 実質的にCreateTodoやUpdateTodoなどがDeserializeとValidateを実装しているか
    T: DeserializeOwned + Validate,
    // どうやらSendは所有権の移動、Syncは参照の移動
    S: Send + Sync,
{
    // エラー時の戻り値の型
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

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = repository.all().await;
    match res {
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Ok(todos) => Ok((StatusCode::OK, Json(todos))),
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
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
    }
}

pub async fn create_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository
        .create(payload)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let res = repository.update(id, payload).await;

    match res {
        Err(err) => {
            println!("{err}");
            match err {
                RepositoryError::NotFound(_) => Err(StatusCode::NOT_FOUND),
                RepositoryError::Unexpected(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
    }
}

pub async fn delete_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<i32>,
) -> StatusCode {
    let res = repository.delete(id).await;

    match res {
        Err(err) => match err {
            RepositoryError::NotFound(_) => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Ok(_) => StatusCode::NO_CONTENT,
    }
}
