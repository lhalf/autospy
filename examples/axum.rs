//! Test an [`axum`](https://docs.rs/axum/latest/axum/) HTTP server which manages a filesystem.

use anyhow::Context;
use async_trait::async_trait;
use axum::Router;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::put;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .context("failed to bind to listening address")?;

    axum::serve(listener, app(Filesystem {}))
        .await
        .context("failed to start server")
}

fn app(filesystem: impl SaveFile) -> Router {
    Router::new()
        .route("/{filename}", put(handle_request))
        .with_state(Arc::new(filesystem))
}

async fn handle_request(
    Path(file): Path<String>,
    State(file_saver): State<Arc<dyn SaveFile>>,
    body: Bytes,
) -> StatusCode {
    match file_saver.save_file(file, &body).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg_attr(test, autospy::autospy)]
#[async_trait]
trait SaveFile: Send + Sync + 'static {
    async fn save_file(&self, filename: String, contents: &[u8]) -> Result<(), anyhow::Error>;
}

#[derive(Clone)]
struct Filesystem {}

#[async_trait]
impl SaveFile for Filesystem {
    async fn save_file(&self, filename: String, contents: &[u8]) -> Result<(), anyhow::Error> {
        std::fs::write(&filename, contents).context(format!("failed to save file {filename}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn failing_to_save_file_returns_500() {
        let save_file_spy = SaveFileSpy::default();
        save_file_spy
            .save_file
            .returns
            .set([Err(anyhow::anyhow!("deliberate test error"))]);

        let response = app(save_file_spy.clone())
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/filename")
                    .body(Body::from("file contents"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn saving_a_file_returns_200_and_uses_correct_filename_and_contents() {
        let save_file_spy = SaveFileSpy::default();
        save_file_spy.save_file.returns.set([Ok(())]);

        let response = app(save_file_spy.clone())
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri("/filename")
                    .body(Body::from("file contents"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            save_file_spy.save_file.arguments.take(),
            vec![("filename".to_string(), b"file contents".to_vec())]
        );
    }
}
