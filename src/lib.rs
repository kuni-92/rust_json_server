use std::{io::{Read, Write}, net::{TcpListener, ToSocketAddrs}};

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn new<A>(address: A) -> Server
        where A: ToSocketAddrs
    {
        let listener = TcpListener::bind(address).unwrap();
        Server { listener: listener }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buf = [0;1024];
            let _ = stream.read(&mut buf).unwrap();
            let request = std::str::from_utf8(&buf).unwrap();
            println!("{}", request);

            let response = "HTTP/1.1 200 OK \r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
