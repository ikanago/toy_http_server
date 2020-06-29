extern crate toy_http_server;

use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use toy_http_server::server::Server;

fn handle_client(mut stream: TcpStream) {
    println!("Request from: {}", stream.peer_addr().unwrap());
    let mut buf = [0; 512];
    stream.read(&mut buf).expect("Error reading stream");

    let get_start_line = b"GET / HTTP/1.1\r\n";
    let sleep_start_line = b"GET /sleep HTTP/1.1\r\n";
    let (status_message, file_name) = if buf.starts_with(get_start_line) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buf.starts_with(sleep_start_line) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}", status_message, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handler_ok() -> String {
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

fn handler_sleep() -> String {
    thread::sleep(Duration::from_secs(5));
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1";
    let port = 1111;
    Server::new()
        .route("/", handler_ok)
        .route("/sleep", handler_sleep)
        .run()?;
    Ok(())
}
