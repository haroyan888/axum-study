use axum::{routing::get, Router};
use std::env;
use std::path::Path;

mod router;

#[tokio::main]
async fn main() {
    // .envファイルの読み込み
    dotenvy::from_path(Path::new("../.env")).unwrap();

    let host = env::var("APP_HOST").expect("APP_HOSTが設定されていません");
    let port = env::var("APP_PORT").expect("APP_PORTが設定されていません");

    let app = router::create_router();

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app).await.expect("failed to listen");
}

async fn handler() -> &'static str {
    return "Hello, World";
}
