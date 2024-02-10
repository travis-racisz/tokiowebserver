use askama::Template;
use axum::{routing::get, Router};
use config::Config;
use sqlx::{postgres::PgPool, types::chrono::NaiveDateTime};

pub mod config;
#[tokio::main]
async fn main() {
    Config::set_env(); // read the .env file and set the environment variables

    let database_url = std::env::var("DATABASE_URL") // get the database url from the environment
        .expect("could not find database url, please check .env file and provide valid url");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect to database, Check URL"); // create a connection pool

    let app = Router::new()
        .route("/", get(get_base))
        .route("/about", get(about))
        .route("/home", get(home))
        .nest_service(
            "/assets",
            tower_http::services::ServeDir::new(format!(
                "{}/assets",
                std::env::current_dir().unwrap().to_str().unwrap()
            )),
        );

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HTMLTemplate;

async fn get_base() -> BaseTemplate<'static> {
    return BaseTemplate {
        title: "Rust Diablo",
    };
}

async fn home() -> HTMLTemplate {
    return HTMLTemplate;
}
#[derive(Template)]
#[template(path = "about.html")]
struct About<'a> {
    title: &'a str,
}

async fn about() -> About<'static> {
    About { title: "about" }
}

#[derive(Template)]
#[template(path = "posts.html")]
struct Posts {
    title: String,
    posts: Vec<Post>,
}

struct Post {
    id: i32,
    title: String,
    content: String,
    created_at: NaiveDateTime,
}

async fn get_all_posts(pool: PgPool) {
    let posts = sqlx::query!("SELECT * FROM posts where id = $1", 1)
        .fetch_one(&pool)
        .await
        .expect("could not get post");
    println!("{}:", posts.title);
}
