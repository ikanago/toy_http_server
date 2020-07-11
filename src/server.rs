use crate::handler::Handler;
use crate::request::Request;
use crate::router::route::Route;
use std::error::Error;
use std::io::{self, Read, Write};
use std::net;
use std::str::FromStr;

pub struct Server {
    address: String,
    port: u16,
    router: Route,
}

impl Server {
    pub fn new() -> Self {
        let address = "127.0.0.1".to_string();
        let port = 8000u16;
        let router = Route::new();
        Self {
            address,
            port,
            router,
        }
    }

    pub fn bind(self, address: &str, port: u16) -> Self {
        Self {
            address: address.to_string(),
            port,
            ..self
        }
    }

    pub fn route<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Handler,
    {
        self.router.add_route(path, handler);
        self
    }

    fn parse_request(stream: &mut net::TcpStream) -> Result<Request, Box<dyn Error>> {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).expect("Error reading stream");
        // Todo: remove unwrap()
        let buffer = String::from_utf8(buffer.to_vec()).unwrap();
        Request::new(&buffer)
    }

    pub fn run(&mut self) -> io::Result<()> {
        let bound_address = net::IpAddr::from_str(&self.address).unwrap();
        let bound_address = net::SocketAddr::new(bound_address, self.port);
        let listener = net::TcpListener::bind(bound_address)?;
        println!("Server listening on {}", listener.local_addr().unwrap());
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let request = match Self::parse_request(&mut stream) {
                        Ok(request) => request,
                        Err(err) => {
                            eprintln!("{}", err);
                            continue;
                        }
                    };
                    let handler = match self.router.find(&request.uri) {
                        Some(handler) => handler,
                        None => panic!("Handler not set for the route: {}", request.uri),
                    };
                    let response: Vec<u8> = handler.handle(&request).into();
                    dbg!(request);
                    stream.write(&response)?;
                    stream.flush()?;
                }
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
            }
        }
        Ok(())
    }
}
