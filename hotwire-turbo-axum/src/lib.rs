use axum::body::Body;
use axum::http::{header, HeaderValue};
use axum::response::{IntoResponse, Response};

/// A Hotwire TurboStream response.
///
/// Will automatically get `Content-Type: text/vnd.turbo-stream.html`.
#[derive(Clone, Copy, Debug)]
#[must_use]
pub struct TurboStream<T>(pub T);

impl<T> IntoResponse for TurboStream<T>
where
    T: Into<Body>,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/vnd.turbo-stream.html"),
            )],
            self.0.into(),
        )
            .into_response()
    }
}

impl<T> From<T> for TurboStream<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, routing::get, Router};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn test() -> impl IntoResponse {
        TurboStream("Test")
    }

    #[tokio::test]
    async fn content_type() {
        let app = Router::new().route("/test", get(test));

        let res = app
            .clone()
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert!(res.headers()["Content-Type"]
            .to_str()
            .unwrap()
            .contains("text/vnd.turbo-stream.html"));
    }

    #[tokio::test]
    async fn body() {
        let app = Router::new().route("/test", get(test));

        let res = app
            .clone()
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let body = body_text(res).await;
        assert_eq!(body, "Test");
    }

    async fn body_text<B>(body: B) -> String
    where
        B: axum::body::HttpBody,
        B::Error: std::fmt::Debug,
    {
        let bytes = body.collect().await.unwrap().to_bytes();
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}
