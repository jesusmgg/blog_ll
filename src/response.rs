use std::io::Write;
use std::{fs, net::TcpStream};

use crate::tls::TlsConnection;
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

    pub fn send(&self, mut tcp_stream: TcpStream, tls_conn: &mut TlsConnection) {
        let mut stream = tls_conn.writer();

        stream.write(&self.version.as_bytes()).unwrap();
        stream.write(b" ").unwrap();
        stream
            .write(get_status_code_str(&self.status_code).as_bytes())
            .unwrap();
        stream.write(b"\n").unwrap();

        for header in &self.headers {
            stream.write(&header.as_bytes()).unwrap();
            stream.write(b"\n").unwrap();
        }

        stream.write(b"\n").unwrap();
        stream.write(&self.body.as_bytes()).unwrap();

        let _bytes_written = tls_conn.write_tls(&mut tcp_stream);
        // stream.flush().unwrap();
    }
}
