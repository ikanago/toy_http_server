use crate::request::RequestParseError;
use std::collections::HashMap;
use std::str::FromStr;
pub type Headers = HashMap<HeaderField, String>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum HeaderField {
    // Request headers:
    Accept,
    Host,
    UserAgent,
    // Entity headers
    ContentLength,
    ContentType,
    // Unrecognized header field
    Undefined,
}

impl FromStr for HeaderField {
    type Err = RequestParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let header = match s {
            "Accept" => HeaderField::Accept,
            "Host" => HeaderField::Host,
            "User-Agent" => HeaderField::UserAgent,
            "Content-Length" => HeaderField::ContentLength,
            "Content-Type" => HeaderField::ContentType,
            _ => HeaderField::Undefined,
        };
        Ok(header)
    }
}

impl Into<Vec<u8>> for HeaderField {
    fn into(self) -> Vec<u8> {
        let header_field = match self {
            HeaderField::Accept => "Accept",
            HeaderField::Host => "Host",
            HeaderField::UserAgent => "User-Agent",
            HeaderField::ContentLength => "Content-Length",
            HeaderField::ContentType => "Content-Type",
            HeaderField::Undefined => "",
        };
        header_field.as_bytes().to_vec()
    }
}

pub fn to_vec(headers: Headers) -> Vec<u8> {
    let mut headers_vec = Vec::new();
    for (header_field, header_value) in headers {
        let mut header_field: Vec<u8> = header_field.into();
        headers_vec.append(&mut header_field);
        headers_vec.append(&mut ": ".as_bytes().to_vec());
        headers_vec.append(&mut header_value.into_bytes());
        headers_vec.append(&mut "\r\n".as_bytes().to_vec());
    }
    headers_vec
}
