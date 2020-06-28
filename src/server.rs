use std::net;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct Server {
    #[builder(setter())]
    address: net::SocketAddr,
    #[builder(setter())]
    listener: net::TcpListener,
}

impl Server {
    pub fn run<F>(&mut self, mut handler: F)
    where
        F: FnMut(net::TcpStream),
    {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    handler(stream);
                }
                Err(err) => panic!(err),
            }
        }
    }
}
