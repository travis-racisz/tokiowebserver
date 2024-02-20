use askama::Template;
use axum::{routing::get, Extension, Router};
use config::Config;
use database::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::sql::Thing;
mod config;
mod database;
mod pages;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Config::set_env();
    let db = Arc::new(Database::new().await?);
    let app = Router::new()
        .route("/post", get(get_post))
        .route("/about", get(pages::about::get_about))
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: String,
    content: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Record {
    id: Thing,
    title: String,
}
#[derive(Template)]
#[template(path = "posts.html")]
struct PostTemplate {
    title: String,
    posts: Vec<Record>,
}
#[axum::debug_handler]
async fn get_post(Extension(db): Extension<Arc<Database>>) -> PostTemplate {
    let result: Vec<Record> = db.db.select("posts").await.unwrap();
    dbg!(&result);
    PostTemplate {
        title: "Posts!!!".to_string(),
        posts: result,
    }
}
