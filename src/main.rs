use std::fs::File;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};

fn handle_client(stream: &mut TcpStream) {
    println!("Request from: {}", stream.peer_addr().unwrap());
    let mut buf = [0; 512];
    stream.read(&mut buf).expect("Error reading stream");

    let get_start_line = b"GET / HTTP/1.1\r\n";
    let (status_message, file_name) = if buf.starts_with(get_start_line) {
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

fn main() -> std::io::Result<()> {
    let address = SocketAddr::from(([127, 0, 0, 1], 1111));
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                handle_client(&mut s);
            }
            Err(e) => eprintln!("{:?}", e),
        }
    }
    Ok(())
}
