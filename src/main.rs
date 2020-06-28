use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: &mut TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).expect("Error reading stream");
    let result = String::from_utf8(buf.to_vec()).unwrap();
    println!("{}", result);
    println!("All read");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1111")?;
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
