use axum::{routing::get, Extension, Router};
use std::env;
use std::path::Path;
use std::sync::Arc;

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

    let app = create_todo_router(repository);

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
        .nest(
            "/todo",
            Router::new()
                .route("/", get(all_todo::<T>).post(create_todo::<T>))
                .route(
                    "/:id",
                    get(find_todo::<T>)
                        .patch(update_todo::<T>)
                        .delete(delete_todo::<T>),
                ),
        )
        .layer(Extension(Arc::new(repository)))
}
