use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;
use std::path::PathBuf;
use dotenv::dotenv;
use std::env;

mod templates;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let address = format!("{}:{}", host, port);

    // Consider moving this into another directory, aka "routes"
    let name = "Brian";
    let markup = templates::index(name);
    let rendered_html = Html(markup.into_string());

    let serve_dir = ServeDir::new(PathBuf::from("static"));
    let app = Router::new()
        .route("/", get(move || async move { rendered_html.clone() }))
        .nest_service("/static", serve_dir);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Listening on {}", address);
}
