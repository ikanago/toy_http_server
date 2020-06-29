extern crate toy_http_server;

use std::thread;
use std::time::Duration;
use toy_http_server::server::Server;

fn handler_ok() -> String {
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

fn handler_sleep() -> String {
    thread::sleep(Duration::from_secs(5));
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

fn main() -> std::io::Result<()> {
    Server::new()
        .route("/", handler_ok)
        .route("/sleep", handler_sleep)
        .run()?;
    Ok(())
}
