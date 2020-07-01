use std::convert::Into;

/// Enum of response statuses. Each status is converted into status code and message
/// through `into()` function.
pub enum Status {
    OK,
}

impl Into<(u16, String)> for Status {
    fn into(self) -> (u16, String) {
        match self {
            Status::OK => (200, "OK".to_string()),
        }
    }
}
