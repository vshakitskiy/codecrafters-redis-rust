#![allow(unused_imports)]
mod resp_parser;

use std::borrow::Cow;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::{num, thread};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Failed to bind to port 6379");

    loop {
        let (socket, _) = listener
            .accept()
            .expect("Failed to accept incoming connection");

        thread::spawn(|| {
            handle_client(socket);
        });
    }
}

fn handle_client(stream: TcpStream) {
    println!("accepted new connection!");
    let mut reader = BufReader::new(stream);
    let mut buf = [0; 1024];

    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }

        let data = String::from_utf8_lossy(&buf[..n]);

        println!("{:?}", resp_parser::parse_resp_array(data));

        //     to_resp_array(line);

        //     if line.trim() == "PING" {
        //         reader
        //             .get_mut()
        //             .write_all(b"+PONG\r\n")
        //             .expect("Failed to send PONG");
        //     }

        buf.fill(0);
    }
}
