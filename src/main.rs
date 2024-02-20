use axum::{routing::get_service, Router};
use tower_http::{
    services::{ServeDir, ServeFile},
    validate_request::ValidateRequestHeaderLayer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route(
            "/protected",
            get_service(ServeFile::new("public/protected.html")),
        )
        .route_layer(ValidateRequestHeaderLayer::basic("user1", "1234"))
        .route("/", get_service(ServeFile::new("public/index.html")))
        .fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
