use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

pub struct Database {
    pub db: Surreal<Client>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Post {
    title: String,
    content: String,
}
#[derive(Deserialize, Debug)]
struct Record {
    id: Thing,
}

impl Database {
    pub async fn new() -> surrealdb::Result<Self> {
        let config = crate::config::Config::set_env();
        let db = Surreal::new::<Ws>(&config.surreal_url).await?;
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await;
        db.use_ns("rust_test").use_db("test").await;

        Ok(Self { db })
    }

    pub async fn create_post() -> surrealdb::Result<()> {
        let db = Self::new().await?;
        let created: Vec<Record> = db
            .db
            .create("post")
            .content(Post {
                title: "Hello, world!".to_string(),
                content: "This is a post.".to_string(),
            })
            .await?;
        dbg!(created);
        Ok(())
    }
}
