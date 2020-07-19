use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::string::String;

fn main() {
    let address = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        breakdown_request(stream);
    }
}

fn breakdown_request(mut stream: TcpStream) {
    // So the buffer could have a bunch of different types,
    // we want to start with just responding to get and post requests
    // and 404ing the others.
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // eventually we want a function that allows us to assign routes with different requests...
    if buffer.starts_with(b"GET") {
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

    /*
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
    */
}
