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
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(request_str: &str) -> Result<Request, Box<dyn Error>> {
        let request_lines = request_str.split("\r\n").collect::<Vec<&str>>();
        let mut request_lines = request_lines.iter();
        let request_line = request_lines
            .next()
            .ok_or_else(|| RequestParseError::Empty)?;
        let (method, uri) = Self::parse_request_line(*request_line)?;
        Ok(Request {
            method,
            uri,
            headers: HashMap::new(),
        })
    }

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
            .as_str()
            .to_string();
        Ok((method, uri))
    }
}

#[derive(Clone, Debug)]
pub enum RequestParseError {
    Empty,
    InvalidMethod,
    LackingPath,
}

impl std::fmt::Display for RequestParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &RequestParseError::Empty => write!(f, "Empty request"),
            &RequestParseError::InvalidMethod => write!(f, "Invalid type of method"),
            &RequestParseError::LackingPath => write!(f, "Lacking path"),
        }
    }
}

impl Error for RequestParseError {}

#[cfg(test)]
mod tests {
    use crate::request::{Method, Request};

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
}
