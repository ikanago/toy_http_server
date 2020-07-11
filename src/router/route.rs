/// Check if the path has wild card at the end of the path.
use crate::handler::Handler;

fn includes_wildcard(path: &str) -> bool {
    path.ends_with("/*")
}

/// Node of trie tree.
#[derive(Default)]
pub struct Route {
    pub path: String,
    pub handler: Option<Box<dyn Handler>>,
    children: Vec<Box<Route>>,
}

impl Route {
    pub fn new() -> Self {
        Default::default()
    }

    fn new_child<F: Handler>(path: &str, handler: F) -> Self {
        let mut child = Self {
            path: path.to_string(),
            handler: Some(Box::new(handler)),
            children: Vec::new(),
        };
        if includes_wildcard(path) && !path.starts_with('*') {
            child.split_wildcard();
        }
        child
    }

    /// Return how many common character path of `Route` nodes and an arugument have.
    fn longest_common_prefix(&self, other: &str) -> usize {
        let mut pos = 0;
        for (char_self, char_other) in self.path.chars().zip(other.chars()) {
            if char_self == char_other {
                pos += 1;
            } else {
                break;
            }
        }
        pos
    }

    pub fn add_route<F: Handler>(&mut self, new_path: &str, handler: F) {
        // For the first time to insert node to root.
        if self.path.len() == 0 && self.children.len() == 0 {
            self.children
                .push(Box::new(Route::new_child(new_path, handler)));
            return;
        }
        if self.path == new_path {
            self.handler = Some(Box::new(handler));
            return;
        }

        let lcp = self.longest_common_prefix(new_path);
        // If length of longest common prefix is not 0, `self.path` cannot be `None`.
        let path = self.path.clone();
        // For example, `self.path` is "static" and longest common prefix is "stat".
        if path.len() > lcp {
            let common_prefix = &path[..lcp];
            let path_remaining = &path[lcp..];
            let new_path_remaining = &new_path[lcp..];

            let new_child = Self {
                path: path_remaining.to_string(),
                handler: std::mem::take(&mut self.handler),
                children: std::mem::take(&mut self.children),
            };
            self.path = common_prefix.to_string();
            self.children = vec![
                Box::new(new_child),
                Box::new(Route::new_child(new_path_remaining, handler)),
            ]
        } else {
            // When longest common prefix is exactly the same as `self.path`.
            let new_path_remaining = &new_path[lcp..];
            for child in &mut self.children {
                match (*child).path.chars().next() {
                    // Because more than 2 children node do not have same prefix,
                    // just check first character of key for each child.
                    Some(first_char)
                        if first_char == new_path_remaining.chars().next().unwrap() =>
                    {
                        child.add_route(new_path_remaining, handler);
                        return;
                    }
                    _ => continue,
                }
            }
            // If there is no child to match new path, just insert it.
            self.children
                .push(Box::new(Route::new_child(new_path_remaining, handler)));
        }
    }

    fn split_wildcard(&mut self) {
        if includes_wildcard(&self.path) {
            self.path = self.path.trim_end_matches('*').to_string();
            self.children.push(Box::new(Self {
                path: "*".to_string(),
                handler: None,
                children: Vec::new(),
            }));
        }
    }

    pub fn find(&self, key: &str) -> Option<&Box<dyn Handler>> {
        if key.len() == 0 {
            return None;
        }
        let lcp = self.longest_common_prefix(key);
        let key_remaining = &key[lcp..];
        if key_remaining.len() == 0 {
            return self.handler.as_ref();
        }

        for child in &self.children {
            if &child.path == "*" {
                return self.handler.as_ref();
            }
            match (*child).path.chars().next() {
                // Because more than 2 children node do not have same prefix,
                // just check first character of key for each child.
                Some(first_char) if first_char == key_remaining.chars().next().unwrap() => {
                    return child.find(key_remaining);
                }
                _ => continue,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::request::Request;
    use crate::response::Response;
    use crate::router::route::Route;

    #[test]
    fn test_lcp() {
        let node_x = Route {
            path: "abcde".to_string(),
            handler: None,
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix("abchoge"), 3);
    }

    #[test]
    fn test_lcp_root() {
        let node_x = Route {
            path: "".to_string(),
            handler: None,
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix("abchoge"), 0);
    }

    fn dummy_handler(_request: &Request) -> Response {
        unimplemented!()
    }

    #[test]
    fn test_find() {
        let mut tree = Route::new();
        let keys = vec!["/", "to", "tea", "ted", "ten", "i", "in", "inn"];
        for key in &keys {
            tree.add_route(key, dummy_handler);
        }
        for key in keys {
            match tree.find(key) {
                Some(_) => continue,
                None => panic!()
            }
        }
    }

    // Generate random alphanumeric string.
    fn random_string() -> String {
        extern crate rand;
        use rand::distributions::Alphanumeric;
        use rand::random;
        use rand::Rng;
        let length = random::<usize>() % 500 + 1;
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect::<String>()
    }

    #[test]
    fn test_find_random() {
        let mut tree = Route::new();
        let count = 1000;
        let keys = (0..count).map(|_| random_string()).collect::<Vec<String>>();
        for key in &keys {
            tree.add_route(key, dummy_handler);
        }
        for key in &keys {
            match tree.find(key) {
                Some(_) => continue,
                None => panic!("{}", key)
            }
        }
    }

    #[test]
    fn test_find_with_wildcard() {
        let mut tree = Route::new();
        let paths = vec!["/", "/index.html", "/static/*"];
        for key in &paths {
            tree.add_route(key, dummy_handler);
        }
        let queries = vec![
            "/",
            "/index.html",
            "/static/index.html",
            "/static/style.css",
            "/static/index.js",
        ];
        for query in &queries {
            match tree.find(query) {
                Some(_) => continue,
                None => panic!()
            }
        }
    }
}
