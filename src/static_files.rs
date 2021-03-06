use crate::handler::Handler;
use crate::request::Request;
use crate::responder::Responder;
use crate::response::Response;
use std::fs::File;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct StaticFiles {
    root: PathBuf,
}

impl StaticFiles {
    pub fn new<P: AsRef<Path>>(root: P) -> Self {
        let root: PathBuf = root.as_ref().into();
        Self { root }
    }
}

impl Handler for StaticFiles {
    fn handle(&self, request: &Request) -> Response {
        let current_dir = std::env::current_dir().unwrap();
        let request_path = current_dir.join(&self.root);
        dbg!(&request_path);
        let request_path = request_path.join(&request.uri.trim_start_matches('/'));
        dbg!(&request_path);
        // Todo: remove unwrap() and return 404 instead.
        let file = File::open(request_path).unwrap();
        let response = file.to_response().unwrap();
        response
    }
}
