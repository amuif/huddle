pub mod config;

use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

/// Builds the application router. Kept separate from `main` so tests can drive it directly.
pub fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http())
}

async fn root() -> &'static str {
    "main route"
}
async fn health() -> &'static str {
    "ok"
}

/// Symbol exported for `axum-hotreload` to dynamically recreate the router on changes.
#[unsafe(no_mangle)]
pub extern "Rust" fn axum_hotreload_build_router(_state: ()) -> Router<()> {
    app()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_returns_ok() {
        let res = app()
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        let body = res.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"ok");
    }

    #[tokio::test]
    async fn unknown_route_is_404() {
        let res = app()
            .oneshot(Request::get("/nope").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
