use axum::{routing::get, Extension, Router};
use config::Config;
use database::Database;
use std::sync::Arc;
mod config;
mod database;
mod pages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Config::set_env();
    let db = Arc::new(Database::new().await?);
    let app = Router::new()
        .route("/", get(crate::pages::get_index))
        .route("/posts", get(crate::pages::get_posts))
        .route("/about", get(crate::pages::get_about))
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
