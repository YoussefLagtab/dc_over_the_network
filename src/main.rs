use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

mod dc;
use dc::dc;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

		println!("\nServer runing on localhost:8000");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream)
{
	let mut stack: Vec<i32> = Vec::new();

	loop {
		let mut buffer = [0; 512];
		stream.read(&mut buffer).unwrap();
		let line = String::from_utf8_lossy(&buffer[..]).to_string();
		if line.len() > 0 {
			stack = dc(line, stack, &stream);
		}
	}
}
