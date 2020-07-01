use crate::request::RequestParseError;
use std::collections::HashMap;
use std::str::FromStr;
pub type HeaderMap = HashMap<HeaderField, String>;

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

impl Into<Vec<u8>> for &HeaderField {
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

/// Convert hash map of headers into vector of bytes joined by a newline character.
// I tried to implement `Into<Vec<u8>>` for `Headers` but the implementation is reserved.
pub fn to_vec(headers: &HeaderMap) -> Vec<u8> {
    let mut headers_vec = Vec::new();
    for (header_field, header_value) in headers {
        let mut header_field: Vec<u8> = header_field.into();
        headers_vec.append(&mut header_field);
        headers_vec.append(&mut ": ".as_bytes().to_vec());
        headers_vec.append(&mut header_value.as_bytes().to_vec());
        headers_vec.append(&mut "\r\n".as_bytes().to_vec());
    }
    headers_vec
}

#[cfg(test)]
mod tests {
    use crate::headers::{to_vec, HeaderField, HeaderMap};
    use crate::request::Request;

    #[test]
    fn test_to_vec() {
        let headers = [
            (HeaderField::ContentLength, "3".to_string()),
            (HeaderField::UserAgent, "curl/7.58.0".to_string()),
            (HeaderField::Host, "localhost:8000".to_string()),
            (HeaderField::Accept, "*/*".to_string()),
            (
                HeaderField::ContentType,
                "application/x-www-form-urlencoded".to_string(),
            ),
        ]
        .iter()
        .cloned()
        .collect::<HeaderMap>();

        let expected = to_vec(&headers);
        let expected = String::from_utf8(expected).unwrap();
        let expected = expected.split("\r\n").collect::<Vec<&str>>();
        let expected = Request::parse_headers(&expected).unwrap();

        assert_eq!(headers, expected,);
    }
}
