mod request;
mod response;
mod tls;
mod url;

use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use crate::{request::Request, response::Response, tls::TlsConnection, url::handle_url};

fn handle_client(mut stream: TcpStream, tls_conn: &mut TlsConnection) {
    let address = stream.local_addr().unwrap();
    println!("Handling connection from: {}", address);

    let mut _byte_count = tls_conn.read_tls(&mut stream).unwrap();
    let mut data: [u8; 2048] = [0; 2048];
    _byte_count = tls_conn.reader().read(&mut data).unwrap();
    tls_conn.process_new_packets().unwrap();

    // let _byte_count = stream.read(&mut data).unwrap();
    let request = Request::new(&data);
    let (document_path, status_code) = handle_url(&request.path).unwrap();
    println!("Document path: {:?}", document_path);
    println!("Status code: {:?}", status_code);

    let response = Response::new(status_code, &document_path);
    response.send(stream, tls_conn);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:11111")?;
    let mut tls_connection = TlsConnection::new();

    for stream in listener.incoming() {
        handle_client(stream?, &mut tls_connection);
    }

    println!("End of program");
    Ok(())
}
