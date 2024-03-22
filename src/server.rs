use std::io::{Read, Write}; // import std library input/output read and write
use std::net::{TcpListener, TcpStream}; // import std library tcp listener and streaming
use std::thread; // import std library thread, will spawn when tcp connection "binds"

fn handle_client(mut stream: TcpStream) {
    // create function handle client
    let mut buf = [0; 512]; // create buffer
    loop {
        match stream.read(&mut buf) {
            // read bytes from buffer
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    return;
                } // zero bytes, client closed connexction
                stream.write(&buf[0..bytes_read]).unwrap(); // echo back to the client
            }
            Err(e) => {
                // trap error
                eprintln!("Failed to read from client: {}", e); // print error messaage
                return;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3470").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept client: {}", e);
            }
        }
    }
}
