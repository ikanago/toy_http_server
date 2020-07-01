use crate::headers::{HeaderField, Headers};
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
}

impl FromStr for Method {
    type Err = RequestParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "GET" => Method::Get,
            _ => return Err(RequestParseError::InvalidMethod),
        };
        Ok(method)
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub uri: String,
    pub headers: Headers,
}

impl Request {
    pub fn new(request_str: &str) -> Result<Request, Box<dyn Error>> {
        let request_lines = request_str.split("\r\n").collect::<Vec<&str>>();
        let (request_line, header_lines) = request_lines
            .split_first()
            .ok_or_else(|| RequestParseError::Empty)?;
        let (method, uri) = Self::parse_request_line(*request_line)?;
        let headers = Self::parse_headers(header_lines)?;

        Ok(Request {
            method,
            uri,
            headers,
        })
    }

    /// Parse first line of request. Return method type and uri of the request.
    fn parse_request_line(request_line_str: &str) -> Result<(Method, String), Box<dyn Error>> {
        let request_line_regex = Regex::new(r"([A-Z]+) ((/.*)*) HTTP/[1,2].\d{1}")?;
        let caps = request_line_regex
            .captures(request_line_str)
            .ok_or_else(|| RequestParseError::Empty)?;
        let method = caps
            .get(1)
            .ok_or_else(|| RequestParseError::InvalidMethod)?
            .as_str();
        let method = Method::from_str(method)?;
        let uri = caps
            .get(2)
            .ok_or_else(|| RequestParseError::LackingPath)?
            .as_str();
        let uri = uri.trim_start_matches('/').to_string();
        Ok((method, uri))
    }

    /// Parse request lines except for the first line of it and return a map of
    /// header field and its value.
    // Todo: return remaining request lines.
    pub(crate) fn parse_headers(header_lines: &[&str]) -> Result<Headers, RequestParseError> {
        let mut headers = HashMap::new();
        let header_lines = header_lines.to_vec();
        let mut header_lines = header_lines.iter();
        while let Some(header_line) = header_lines.next() {
            if *header_line == "" {
                break;
            }
            let header_line = header_line.split(": ").collect::<Vec<&str>>();
            let header_field = header_line
                .first()
                .ok_or_else(|| RequestParseError::InvalidHeaderFormat)?;
            let header_field = HeaderField::from_str(header_field).unwrap();
            let header_value = header_line
                .iter()
                .nth(1)
                .ok_or_else(|| RequestParseError::InvalidHeaderFormat)?
                .clone();
            // Unrecognized header fields should be ignored.
            match header_field {
                HeaderField::Undefined => continue,
                _ => headers.insert(header_field, header_value.to_string()),
            };
        }
        Ok(headers)
    }
}

#[derive(Clone, Debug)]
pub enum RequestParseError {
    Empty,
    InvalidMethod,
    LackingPath,
    InvalidHeaderFormat,
}

impl std::fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &RequestParseError::Empty => write!(f, "Empty request"),
            &RequestParseError::InvalidMethod => write!(f, "Invalid type of method"),
            &RequestParseError::LackingPath => write!(f, "Lacking path"),
            &RequestParseError::InvalidHeaderFormat => write!(f, "Invalid form of header"),
        }
    }
}

impl Error for RequestParseError {}

#[cfg(test)]
mod tests {
    use crate::request::{HeaderField, Method, Request};
    use std::collections::HashMap;

    #[test]
    fn test_parse_request_line_for_root() {
        let request_line = "GET / HTTP/1.1";
        let (method, path) = Request::parse_request_line(request_line).unwrap();
        assert_eq!(method, Method::Get);
        assert_eq!(path, "/".to_string())
    }

    #[test]
    fn test_parse_request_line_for_index() {
        let request_line = "GET /www/index.html HTTP/1.1";
        let (method, path) = Request::parse_request_line(request_line).unwrap();
        assert_eq!(method, Method::Get);
        assert_eq!(path, "/www/index.html".to_string())
    }

    #[test]
    fn test_parse_headers() {
        let header_lines = [
            "Host: localhost:8000",
            "User-Agent: curl/7.58.0",
            "Accept: */*",
            "Content-Length: 3",
            "Content-Type: application/x-www-form-urlencoded",
        ];
        let headers = Request::parse_headers(&header_lines).unwrap();
        assert_eq!(
            headers,
            [
                (HeaderField::ContentLength, "3".to_string()),
                (HeaderField::UserAgent, "curl/7.58.0".to_string()),
                (HeaderField::Host, "localhost:8000".to_string()),
                (HeaderField::Accept, "*/*".to_string()),
                (
                    HeaderField::ContentType,
                    "application/x-www-form-urlencoded".to_string()
                ),
            ]
            .iter()
            .cloned()
            .collect::<HashMap<HeaderField, String>>()
        );
    }
}
