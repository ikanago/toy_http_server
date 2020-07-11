extern crate toy_http_server;

use std::thread;
use std::time::Duration;
use toy_http_server::request::Request;
use toy_http_server::response::Response;
use toy_http_server::server::Server;
use toy_http_server::static_files::StaticFiles;
use toy_http_server::status::Status;

fn handler_sleep(_request: &Request) -> Response {
    thread::sleep(Duration::from_secs(2));
    Response::new(Status::OK)
}

fn main() -> std::io::Result<()> {
    Server::new()
        .route("/*", StaticFiles::new("static"))
        .route("/sleep", handler_sleep)
        .run()?;
    Ok(())
}
