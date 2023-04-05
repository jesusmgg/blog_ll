mod request;
mod response;
mod url;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{request::Request, response::Response, url::handle_url};

fn handle_client(mut stream: TcpStream) {
    let address = stream.local_addr().unwrap();
    println!("Handling connection from: {}", address);

    let mut data: [u8; 2048] = [0; 2048];
    let _byte_count = stream.read(&mut data).unwrap();
    let request = Request::new(&data);
    let (document_path, status_code) = handle_url(&request.path).unwrap();
    println!("Document path: {:?}", document_path);
    println!("Status code: {:?}", status_code);

    let response = Response::new(status_code, &document_path);
    stream.write(&response.to_string().as_bytes()).unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:11111")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    println!("End of program");
    Ok(())
}
