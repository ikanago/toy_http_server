use crate::response::Response;

pub trait Handler: 'static {
    fn handle(&self) -> Response;
}

impl<F: 'static> Handler for F
where
    F: Fn() -> Response,
{
    fn handle(&self) -> Response {
        self()
    }
}
