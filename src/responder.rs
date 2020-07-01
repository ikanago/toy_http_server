use crate::response::Response;
use crate::status::Status;
use std::fs::File;
use std::io::{self, Read};

pub trait Responder {
    fn to_response(self) -> io::Result<Response>;
}

impl Responder for File {
    fn to_response(mut self) -> io::Result<Response> {
        let mut response = Response::new(Status::OK);
        let mut file_content = String::new();
        self.read_to_string(&mut file_content)?;
        response.set_body(file_content);
        Ok(response)
    }
}
