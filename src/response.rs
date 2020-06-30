use crate::status::Status;
use std::convert::Into;

pub struct Response {
    pub status_code: u16,
    pub reason_phrase: String,
}

impl Response {
    pub fn new(status: Status) -> Self {
        let (status_code, reason_phrase) = status.into();
        Self {
            status_code,
            reason_phrase,
        }
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        // Consider to implement `fmt::Display` for `Status`.
        format!("HTTP/1.1 {} {}\r\n", self.status_code, self.reason_phrase)
    }
}
