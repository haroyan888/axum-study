use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, Router},
};

use crate::repositories::Todo;

pub fn create_router() -> Router {
    return Router::new().route("/", get(routing_get_all_todo));
}

fn routing_get_all_todo() -> Result<impl IntoResponse, StatusCode> {
    todos = repository.all();
    Ok((todos, StatusCode::OK))
}
