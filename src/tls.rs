use std::{
    fs,
    io::{Error, Read, Write},
    sync::Arc,
};

use rustls::{
    server::{ServerConfig, ServerConnection},
    Certificate, IoState, PrivateKey, Reader, Writer,
};

pub struct TlsConnection {
    server_connection: ServerConnection,
}

impl TlsConnection {
    pub fn new() -> Self {
        let cert_path = "keys/SelfSigned.crt";
        let key_path = "keys/self_signed.pem";

        let cert_chain = Certificate(fs::read_to_string(cert_path).unwrap().as_bytes().to_vec());
        let key = PrivateKey(fs::read_to_string(key_path).unwrap().as_bytes().to_vec());

        let server_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(vec![cert_chain; 1], key)
            .expect("Bad certificate/key");

        let server_connection = ServerConnection::new(Arc::new(server_config)).unwrap();

        Self { server_connection }
    }

    pub fn read_tls(&mut self, rd: &mut dyn Read) -> Result<usize, Error> {
        self.server_connection.read_tls(rd)
    }
    pub fn write_tls(&mut self, rw: &mut dyn Write) -> Result<usize, Error> {
        self.server_connection.write_tls(rw)
    }

    pub fn process_new_packets(&mut self) -> Result<IoState, rustls::Error> {
        self.server_connection.process_new_packets()
    }

    pub fn reader(&mut self) -> Reader {
        self.server_connection.reader()
    }
    pub fn writer(&mut self) -> Writer {
        self.server_connection.writer()
    }
}
