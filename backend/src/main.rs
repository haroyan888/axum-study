use axum::{routing::get, Extension, Router};
use http::{method::Method, HeaderValue};
use std::env;
use std::path::Path;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod handlers;
use handlers::{all_todo, create_todo, delete_todo, find_todo, update_todo};
mod repositories;
use repositories::TodoRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .envファイルの読み込み
    dotenvy::from_path(Path::new("../.env")).unwrap();

    let host = env::var("APP_HOST").expect("APP_HOSTが設定されていません");
    let port = env::var("APP_PORT").expect("APP_PORTが設定されていません");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URLが設定されていません");
    let repository = repositories::TodoRepositoryForDB::new(&database_url).await?;

    let app = Router::new()
        .nest(
            "/api",
            Router::new().nest("/todo", create_todo_router(repository)),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_headers(Any)
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()),
        );

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app).await.expect("failed to listen");
    Ok(())
}

fn create_todo_router<T>(repository: T) -> Router
where
    T: TodoRepository + 'static,
{
    Router::new()
        .route("/", get(all_todo::<T>).post(create_todo::<T>))
        .route(
            "/search/:id",
            get(find_todo::<T>)
                .patch(update_todo::<T>)
                .delete(delete_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
}
