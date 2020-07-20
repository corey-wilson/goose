use std::net::TcpListener;
use std::string::String;
mod logging;

fn main() {
    let address = String::from("127.0.0.1:7878");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        logging::print_request_info(stream);
        // handle_request(/*stream*/);
    }
}

fn handle_request(/*mut stream: TcpStream*/) {
    println!("handling request");
}

