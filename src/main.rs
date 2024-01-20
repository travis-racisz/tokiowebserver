use askama::Template;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    println!(
        "{}/assets",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let app = Router::new()
        .nest("/api", Router::new().route("/hello", get(say_hello)))
        .route("/", get(hello_world))
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
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

async fn say_hello() -> &'static str {
    "Hello"
}

async fn hello_world() -> HelloTemplate<'static> {
    return HelloTemplate { name: "world" };
}
