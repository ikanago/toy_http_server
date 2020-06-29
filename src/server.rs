use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::net;
use std::str::FromStr;

// #[derive(Debug, Default)]
pub struct Server
// where
    // F: Fn() -> String,
{
    address: String,
    port: u16,
    router: HashMap<String, Box<dyn Fn() -> String>>,
}

impl Server
// where
    // F: Fn() -> String,
{
    pub fn new() -> Self {
        let address = "127.0.0.1".to_string();
        let port = 8000u16;
        let router = HashMap::new();
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
        F: Fn() -> String + 'static
    {
        self.router.insert(path.to_string(), Box::new(handler));
        self
    }

    fn parse_request(stream: &mut net::TcpStream) -> io::Result<String> {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).expect("Error reading stream");
        // Todo: remove unwrap()
        let buffer = String::from_utf8(buffer.to_vec()).unwrap();
        let buffer = buffer.split(' ').collect::<Vec<&str>>();
        Ok(buffer[1].to_string())
    }

    pub fn run(&mut self) -> io::Result<()> {
        // Todo: remove unwrap()
        let bound_address = net::IpAddr::from_str(&self.address).unwrap();
        let bound_address = net::SocketAddr::new(bound_address, self.port);
        let listener = net::TcpListener::bind(bound_address)?;
        println!("Server listening on {}", listener.local_addr().unwrap());
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let path = Self::parse_request(&mut stream)?;
                    let handler = match self.router.get(&path) {
                        Some(handler) => handler,
                        None => panic!("Handler not set for the route: {}", path),
                    };
                    let response = handler();
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
            }
        }
        Ok(())
    }
}

// #[derive(Debug)]
// pub struct Server<F>
// where
//     F: FnMut(net::TcpStream)
// {
//     address: net::SocketAddr,
//     handler: F,
//     listener: net::TcpListener,
//     builder: ServerBuilder,
// }

// impl<F> Server<F>
// where
//     F: FnMut(net::TcpStream)
// {

//     pub fn bind(self, address: &str, port: u16) -> Self {
//         let address = net::IpAddr::from_str(address).unwrap();
//         let address = net::SocketAddr::new(address, port);
//         Self { address, ..self }
//     }

//     pub fn set_handler(self, handler: F) -> Self {
//         Self { handler, ..self }
//     }

//     pub fn listen(self) -> Self {
//         let listener = net::TcpListener::bind(self.address).unwrap();
//         Self { listener, ..self }
//     }

//     pub fn run(&mut self) {
//         for stream in self.listener.incoming() {
//             match stream {
//                 Ok(stream) => {
//                     (self.handler)(stream);
//                 }
//                 Err(err) => panic!(err),
//             }
//         }
//     }
// }
