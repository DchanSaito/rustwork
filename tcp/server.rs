use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let host_ip = "127.0.0.1:3333";
    let listener = TcpListener::bind(host_ip).unwrap();

    println!("server running {}", host_ip);

    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("Accept Error {}", e); }
            Ok(stream) => { handle_client(stream); }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        let len = match stream.read(&mut buffer) {
            Err(_) => break,
            Ok(message) => {
                if message == 0 { break; }
                message
            }
        };
        match stream.write_all(&buffer[..len]) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}
