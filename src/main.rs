use std::net::TcpListener;
use std::string::String;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

mod logging;

fn main() {
    let address = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //TODO: how to enable both?
        //logging::print_request_info(stream);
        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream) {
    // TODO: need to find a good way to handle these different requests
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let (status_line, filename) = if buffer.starts_with(b"GET / H") {
        ("HTTP/1.1 200 OK\r\n\r\n", "html/hello.html")
    } else if buffer.starts_with(b"GET /api H") {
        ("HTTP/1.1 200 OK\r\nContent-Type:application/json\r\n\r\n", "html/all.json")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

