use std::net::TcpStream;
use std::io::prelude::*;


pub fn print_request_info(mut stream: TcpStream) {
    // So the buffer could have a bunch of different types,
    // we want to start with just responding to get and post requests
    // and 404ing the others.
    println!("JUST DO IT");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // eventually we want a function that allows us to assign routes with different requests...
    if buffer.starts_with(b"GET") {
        println!("IN THA GET");
        let buf_string = String::from_utf8_lossy(&buffer[..]);
        let buf_split = buf_string.split("\r\n");
        println!("A GET request was made.");
        let mut i = 0;
        for item in buf_split {
            // on the first pass is the HTTP request
            // on the second pass we have headers
            if i == 0 {
                let request = item.split(" ");
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
            } else if i == 1 {
                println!("--HEADERS--");
                println!("{}", item);
            } else {
                println!("{}", item);
            }
            i += 1;
        }
    }
    println!("OUT THE GET");
}
