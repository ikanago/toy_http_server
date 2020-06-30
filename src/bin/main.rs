extern crate toy_http_server;

use std::thread;
use std::time::Duration;
use toy_http_server::response::Response;
use toy_http_server::server::Server;
use toy_http_server::status::Status;

fn handler_ok() -> Response {
    Response::new(Status::OK)
}

fn handler_sleep() -> Response {
    thread::sleep(Duration::from_secs(5));
    Response::new(Status::OK)
}

fn main() -> std::io::Result<()> {
    Server::new()
        .route("/", handler_ok)
        .route("/sleep", handler_sleep)
        .run()?;
    Ok(())
}
