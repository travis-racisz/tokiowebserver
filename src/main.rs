use anyhow::{Context, Result};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tokiowebserver=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("tracing initialized");

    let router = Router::new()
        .route("/", get(hello))
        .route("/hello/:name", get(hello));

    info!("Router initialized");

    let tcp_listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    axum::serve(tcp_listener, router)
        .await
        .context("error while starting server");

    Ok(())
}

async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {
        name: "from a rust server",
    };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => {
                StatusCode::INTERNAL_SERVER_ERROR;
                format!("Error rendering template: {}", e).into_response()
            }
        }
    }
}
