use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get_service,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

use base64::{prelude::BASE64_STANDARD as basestd, Engine};

enum AuthError {
    FailToParseHeader,
    WrongCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            _ => (
                StatusCode::UNAUTHORIZED,
                [(header::WWW_AUTHENTICATE, "Basic")],
            )
                .into_response(),
        }
    }
}

async fn require_basic_auth(req: Request, next: Next) -> Result<Response, AuthError> {
    let auth = req
        .headers()
        .get(header::AUTHORIZATION)
        .map(|v| v.to_str().unwrap())
        .ok_or(AuthError::FailToParseHeader)?;

    let parts: Vec<_> = auth.split(" ").collect();

    if parts[0] == "Basic" && parts[1] == basestd.encode(b"user1:1234") {
        Ok(next.run(req).await)
    } else {
        Err(AuthError::WrongCredentials)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route(
            "/protected",
            get_service(ServeFile::new("public/protected.html")),
        )
        .route_layer(middleware::from_fn(require_basic_auth))
        .route("/", get_service(ServeFile::new("public/index.html")))
        .fallback_service(ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await?;

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
