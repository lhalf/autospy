//! Test a [`rocket`](https://rocket.rs/) server which manages an upstream request client.

use anyhow::Context;
use async_trait::async_trait;
use rocket::http::Status;
use rocket::{Build, Rocket, State, get, routes};

#[rocket::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = reqwest::Client::new();

    build(Box::new(client))
        .launch()
        .await
        .context("failed to start server")?;

    Ok(())
}

fn build(client: Box<dyn MakeUpstreamRequest>) -> Rocket<Build> {
    rocket::build()
        .manage(client)
        .mount("/", routes![handle_request])
}

#[get("/<path>")]
async fn handle_request(path: &str, client: &State<Box<dyn MakeUpstreamRequest>>) -> Status {
    client
        .make_upstream_request(path)
        .await
        .unwrap_or(Status::BadGateway)
}

#[cfg_attr(test, autospy::autospy)]
#[async_trait]
trait MakeUpstreamRequest: Send + Sync {
    async fn make_upstream_request(&self, path: &str) -> Result<Status, anyhow::Error>;
}

#[async_trait]
impl MakeUpstreamRequest for reqwest::Client {
    async fn make_upstream_request(&self, path: &str) -> Result<Status, anyhow::Error> {
        Ok(Status::new(
            self.get(format!("http://upstream_address:8000/{path}"))
                .send()
                .await
                .context("failed to send request")?
                .status()
                .as_u16(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;

    #[test]
    fn failing_to_send_upstream_request_returns_502() {
        let client_spy = MakeUpstreamRequestSpy::default();
        client_spy
            .make_upstream_request
            .returns
            .push_back(Err(anyhow::anyhow!("deliberate test error")));

        let server = Client::tracked(build(Box::new(client_spy))).unwrap();

        let response = server.get("/path").dispatch();

        assert_eq!(response.status(), Status::BadGateway);
    }

    #[test]
    fn sending_an_upstream_request_uses_correct_path_and_returns_the_upstream_response_status() {
        let client_spy = MakeUpstreamRequestSpy::default();
        client_spy
            .make_upstream_request
            .returns
            .push_back(Ok(Status::ImATeapot));

        let server = Client::tracked(build(Box::new(client_spy.clone()))).unwrap();

        let response = server.get("/path").dispatch();

        assert_eq!(response.status(), Status::ImATeapot);
        assert_eq!(
            client_spy.make_upstream_request.arguments.take_all(),
            vec!["path"]
        );
    }
}
