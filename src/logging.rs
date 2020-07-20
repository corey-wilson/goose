use std::io::prelude::*;
use std::net::TcpStream;

pub fn print_request_info(mut stream: TcpStream) {
    // some janky request printer to break down the http request

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // eventually we want a function that allows us to assign routes with different requests...
    if buffer.starts_with(b"GET") {
        let buf_string = String::from_utf8_lossy(&buffer[..]);
        print_get_request(&buf_string);
    }
    // TODO: add other request types (prob make it generic)?
}

fn print_get_request(request_string: &str) {
    println!("A GET request was made.");
    let buf_split = request_string.split("\r\n");
    let mut i = 0;
    for item in buf_split {
        // on the first line is the HTTP request
        // on the second line (and after) we have headers
        if i == 0 {
            print_get_request_line(&item);
        } else if i == 1 {
            println!("{}", item);
        } else {
            println!("{}", item);
        }
        i += 1;
    }
}

fn print_get_request_line(request_line: &str) {
    let request = request_line.split(" ");
    let mut j = 0;
    for req_item in request {
        if j == 0 {
            println!("The Request Type is: {}", req_item);
        } else if j == 1 {
            println!("The Endpoint Requested is: {}", req_item);
        } else if j == 2 {
            println!("The HTTP Version is: {}", req_item);
        }
        j += 1;
    }
}
