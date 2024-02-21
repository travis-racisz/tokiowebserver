use std::sync::Arc;

use askama::Template;
use axum::Extension;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

use crate::database::Database;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub id: Thing,
    pub title: String,
}
#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostTemplate {
    title: String,
    pub posts: Vec<Record>,
}
#[axum::debug_handler]
pub async fn get_posts(Extension(db): Extension<Arc<Database>>) -> PostTemplate {
    let result: Vec<Record> = db.db.select("posts").await.unwrap();
    match result.len() {
        0 => {
            let record = Record {
                id: Thing {
                    tb: ("posts".to_string()),
                    id: (Id::from(0)),
                },
                title: "No posts found".to_string(),
            };
            return PostTemplate {
                title: "Posts".to_string(),
                posts: vec![record],
            };
        }
        _ => {
            return PostTemplate {
                title: "Posts".to_string(),
                posts: result,
            }
        }
    };
}
