#![allow(unused_imports)]
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to port 6379");

    for stream in listener.incoming() {
        handle_client(stream.expect("Failed to accept incoming connection"));
    }
}

fn handle_client(stream: TcpStream) {
    println!("accepted new connection!");
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    while let Ok(n) = reader.read_line(&mut line) {
        if n == 0 {
            break;
        }

        if line.trim() == "PING" {
            reader
                .get_mut()
                .write_all(b"+PONG\r\n")
                .expect("Failed to send PONG");
        }

        line.clear();
    }
}
