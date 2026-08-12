mod path_resolver;
mod handlers;

use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::net::TcpStream;

fn collect_http_request(buf_reader: BufReader<&TcpStream>) -> Vec<String> {
    let mut http_request: Vec<String> = Vec::new();
    for line in buf_reader.lines(){
        let line = line.unwrap();
        if line.is_empty(){
            break
        }
        http_request.push(line)
    }

    http_request
}

fn handle_request(path: &String, http_request: &Vec<String>) -> String{
    match path.as_str(){
        "" => String::from("HTTP/1.1 200 OK\r\n\r\n"),
        "echo" => handlers::echo(http_request),
        _ => String::from("HTTP/1.1 404 Not Found\r\n\r\n")
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let path_resolver = path_resolver::Paths::new();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let mut _stream = _stream;
                let buf_reader = BufReader::new(&_stream);
                let http_request = collect_http_request(buf_reader);
                let path = path_resolver.get_path_from_request(&http_request);
                let response;
                let handler_response;
                if path_resolver.is_exists(&path) {
                    handler_response = handle_request(&path, &http_request);
                    response = handler_response.as_str();
                }else{
                    response = "HTTP/1.1 404 Not Found\r\n\r\n";
                }
                _stream.write_all(response.as_bytes())?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }


    }
    Ok(())
}
