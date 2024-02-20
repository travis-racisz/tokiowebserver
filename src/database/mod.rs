use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub struct Database {
    pub db: Surreal<Client>,
}

impl Database {
    pub async fn new() -> surrealdb::Result<Self> {
        let config = crate::config::Config::set_env();
        let db = Surreal::new::<Ws>(&config.surreal_url).await?;
        db.signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
        db.use_ns("test").use_db("test").await.unwrap();

        Ok(Self { db })
    }
}
