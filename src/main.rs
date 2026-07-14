mod path_resolver;

use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;



fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let path_resolver = path_resolver::Paths::new();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let mut _stream = _stream;
                println!("accepted new connection");
                let buf_reader = BufReader::new(&_stream);
                let http_request: Vec<_> = buf_reader.lines().map(|line| line.unwrap()).collect();
                let path = path_resolver.get_path_from_request(&http_request);
                let mut response = "";
                if path_resolver.is_exists(&path) {
                    response = "HTTP/1.1 200 OK\r\n\r\n";
                }else{
                    response = "HTTP/1.1 404 Not Found\r\n\r\n";
                }
                _stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
