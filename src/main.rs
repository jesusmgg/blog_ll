use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let address = stream.local_addr().unwrap();
    println!("Handling connection from: {}", address);

    let mut data: [u8; 2048] = [0; 2048];
    let byte_count = stream.read(&mut data).unwrap();
    let data_string = String::from_utf8_lossy(&data);

    println!("Data: {:?}", &data[..byte_count]);
    println!("\nData string:\n{}\n", &data_string);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:11111")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }

    println!("End of program");
    Ok(())
}
