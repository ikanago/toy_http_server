use std::convert::Into;

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
