use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let address = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    
    let home = b"GET / HTTP/1.1\r\n";
    let all = b"GET /all HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(home) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(all) {
        ("HTTP/1.1 200 OK\r\nContent-Type:application/json\r\n\r\n", "all.json")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

enum RequestMethods{
    GET = "GET",
    POST = "POST",


}
