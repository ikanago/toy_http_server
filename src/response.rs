use crate::headers::{HeaderField, Headers};
use crate::status::Status;
use std::collections::HashMap;
use std::convert::Into;

pub struct Response {
    pub status_code: u16,
    pub reason_phrase: String,
    pub body: Option<Vec<u8>>,
    pub headers: Headers,
}

impl Response {
    pub fn new(status: Status) -> Self {
        let (status_code, reason_phrase) = status.into();
        Self {
            status_code,
            reason_phrase,
            body: None,
            headers: HashMap::new(),
        }
    }

    pub fn set_body(&mut self, body: String) {
        let body = body.into_bytes();
        let length = body.len();
        self.headers
            .insert(HeaderField::ContentLength, length.to_string());
        self.headers
            .insert(HeaderField::ContentType, "text/html".to_string());
        self.body = Some(body);
    }
}

impl Into<Vec<u8>> for Response {
    fn into(self) -> Vec<u8> {
        // Consider to implement `fmt::Display` for `Status`.
        let mut response = Vec::new();
        let status_line = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.reason_phrase);
        response.append(&mut status_line.into_bytes());
        for (header_field, header_value) in self.headers {
            let mut header_field: Vec<u8> = header_field.into();
            response.append(&mut header_field);
            response.append(&mut ": ".as_bytes().to_vec());
            response.append(&mut header_value.into_bytes());
            response.append(&mut "\r\n".as_bytes().to_vec());
        }
        response.append(&mut "\r\n".as_bytes().to_vec());
        if let Some(mut body) = self.body {
            response.append(&mut body);
        }
        response
    }
}
