use std::fs;

use crate::url::{get_status_code_str, StatusCode};

pub struct Response {
    pub version: String,
    pub status_code: StatusCode,
    pub headers: Vec<String>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: StatusCode, path: &str) -> Self {
        let version = String::from("HTTP/1.1");

        let mut headers = Vec::<String>::new();
        let content_type = String::from("Content-Type: text/html; charset=utf-8");
        let server = String::from("Server: blog_ll");
        headers.push(content_type);
        headers.push(server);

        let body = fs::read_to_string(path).unwrap();

        Self {
            version,
            status_code,
            headers,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.version);
        output.push(' ');
        output.push_str(get_status_code_str(&self.status_code));
        output.push('\n');

        for header in &self.headers {
            output.push_str(&header);
            output.push('\n');
        }

        output.push('\n');
        output.push_str(&self.body);

        output
    }
}
