use axum::{routing::get_service, Router};
use tower_http::{
    services::{ServeDir, ServeFile},
    validate_request::ValidateRequestHeaderLayer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get_service(ServeFile::new("public/index.html")))
        .fallback_service(ServeDir::new("public"))
        .layer(ValidateRequestHeaderLayer::basic("user1", "1234"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
