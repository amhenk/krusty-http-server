use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

enum RequestMethod {
    Get,
    Post,
    Put,
    Options,
    Delete,
}

struct Request {
    path: String,
    method: RequestMethod,
}

fn parse_request(mut stream: TcpStream) {
    println!("parsing request...");

    let mut request_buffer = [0; 4096];
    let raw_request: String;
    match stream.read(&mut request_buffer[..]) {
        Err(why) => panic!("couldn't read stream!!!\n{}\n", why),
        Ok(read) => {
            raw_request = request_buffer[0..read]
                .escape_ascii()
                .to_string();
        },
    }

    raw_request.trim().split("\\r\\n").for_each(move |part| {
        println!(">> {}", part);
    });

    println!("request parsed...");

    // probably want to return a result of some sort
}

fn write_response(mut stream: TcpStream) {
    // Create a path to the desired file
    let path = Path::new("./src/hello.html");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (), // print!("{} contains:\n{}", display, s),
    }
    let _ = stream.write(b"HTTP/1.1 200\n");
    let _ = stream.write(b"Content-Type: text/html\n\n");

    let _ = stream.write(s.as_str().as_bytes());

}

fn handle_client(stream: TcpStream) {
    stream.take_error().expect("No error was expected..");

    parse_request(stream.try_clone().unwrap());
    write_response(stream.try_clone().unwrap());

    stream
        .shutdown(std::net::Shutdown::Both)
        .expect("shutdown call failed");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Processing Request");
                handle_client(stream);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                println!("idk what this but wait_for_fd() I guess?");
                continue;
            }
            Err(e) => panic!("encountered IO error: {e}"),
        }
    }
    Ok(())
}
