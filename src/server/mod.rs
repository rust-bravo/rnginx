use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;

pub struct Server<'a> {
    name: &'a str,
    addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(name: &'a str, addr: &'a str) -> Server<'a> {
        Server {
            name,
            addr,
        }
    }

    pub fn boot(&self) {
        println!("Starting name: {}", self.name);
        println!("bind address: {}", self.addr);
        let listener = self.bind();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn bind(&self) -> TcpListener {
        TcpListener::bind(self.addr).unwrap()
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP/1.1\r\n";
        if buffer.starts_with(get) {
            let mut file = File::open("hello.html").unwrap();

            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let response = format!("HTTP/1.1 200 OK\r\n\rn{}", contents);

            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
            println!("TODO: Sorry");
        }
    }
}
