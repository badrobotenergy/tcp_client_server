use std::io::{stdin, Read, Write};
use std::net::TcpStream;
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3470").unwrap();
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        stream.write(input.as_bytes()).unwrap();
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer[..]));
    }
}
