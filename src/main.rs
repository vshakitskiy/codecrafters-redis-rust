#![allow(unused_imports)]
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to port 6379");

    for stream in listener.incoming() {
        handle_client(stream.expect("Failed to accept incoming connection"));
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("accepted new connection!");

    stream
        .write_all(b"+PONG\r\n")
        .expect("Failed to send a message");
}
