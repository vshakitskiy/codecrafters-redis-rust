#![allow(unused_imports)]
mod resp;

use std::borrow::Cow;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::{num, thread};

use crate::resp::encode_bulk_string;

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

        let vec = match resp::parse_resp_array(data) {
            Ok(vec) => vec,
            Err(_) => continue,
        };

        if vec[0].to_lowercase() == "ping" {
            reader
                .get_mut()
                .write_all(b"+PONG\r\n")
                .expect("Failed to send PONG");
        } else if vec[0].to_lowercase() == "echo" {
            reader
                .get_mut()
                .write_all(encode_bulk_string(vec[1].clone()).as_bytes())
                .expect("Failed to echo");
        }

        buf.fill(0);
    }
}
