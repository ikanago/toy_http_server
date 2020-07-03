use crate::response::Response;
use crate::request::Request;

pub trait Handler: 'static {
    fn handle(&self, request: &Request) -> Response;
}

impl<F: 'static> Handler for F
where
    F: Fn(&Request) -> Response,
{
    fn handle(&self, request: &Request) -> Response {
        self(request)
    }
}
