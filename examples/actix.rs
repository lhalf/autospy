use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use anyhow::Context;
use async_trait::async_trait;
#[cfg(test)]
use autospy::autospy;

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Filesystem {}))
            .route("/{filename}", web::put().to(handle_request))
    })
    .bind(("0.0.0.0", 8000))
    .context("failed to bind to listening address")?
    .run()
    .await
    .context("failed to start server")
}

async fn handle_request(
    file: web::Path<String>,
    body: web::Bytes,
    file_saver: web::Data<dyn SaveFile>,
) -> impl Responder {
    match file_saver.save_file(file.into_inner(), &body).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
    .finish()
}

#[cfg_attr(test, autospy)]
#[async_trait]
trait SaveFile {
    async fn save_file(&self, filename: String, contents: &[u8]) -> Result<(), anyhow::Error>;
}

#[derive(Clone)]
struct Filesystem {}

#[async_trait]
impl SaveFile for Filesystem {
    async fn save_file(&self, filename: String, contents: &[u8]) -> Result<(), anyhow::Error> {
        std::fs::write(&filename, contents).context(format!("failed to save file {}", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, dev::Service, http, test, web};
    use std::sync::Arc;

    #[actix_web::test]
    async fn failing_to_save_file_returns_500() {
        let save_file_spy = SaveFileSpy::default();
        save_file_spy
            .save_file
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(
                    Arc::new(save_file_spy.clone()) as Arc<dyn SaveFile>
                ))
                .route("/{filename}", web::put().to(handle_request)),
        )
        .await;

        let response = app
            .call(
                test::TestRequest::put()
                    .uri("/filename")
                    .set_payload("file contents")
                    .to_request(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn saving_a_file_returns_200_and_uses_correct_filename_and_contents() {
        let save_file_spy = SaveFileSpy::default();
        save_file_spy.save_file.returns.push_back(Ok(()));

        let app = test::init_service(
            App::new()
                .app_data(web::Data::from(
                    Arc::new(save_file_spy.clone()) as Arc<dyn SaveFile>
                ))
                .route("/{filename}", web::put().to(handle_request)),
        )
        .await;

        let response = app
            .call(
                test::TestRequest::put()
                    .uri("/filename")
                    .set_payload("file contents")
                    .to_request(),
            )
            .await
            .unwrap();
        assert!(response.status().is_success());
        assert_eq!(
            save_file_spy.save_file.arguments.take_all(),
            vec![("filename".to_string(), b"file contents".to_vec())]
        );
    }
}
