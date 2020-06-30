extern crate toy_http_server;

use std::thread;
use std::time::Duration;
use toy_http_server::response::Response;
use toy_http_server::server::Server;
use toy_http_server::status::Status;

fn handler_ok() -> Response {
    let mut response = Response::new(Status::OK);
    response.set_body("<h1>Hello<h1>\n".to_string());
    response
}

fn handler_sleep() -> Response {
    thread::sleep(Duration::from_secs(2));
    Response::new(Status::OK)
}

fn main() -> std::io::Result<()> {
    Server::new()
        .route("/", handler_ok)
        .route("/sleep", handler_sleep)
        .run()?;
    Ok(())
}
