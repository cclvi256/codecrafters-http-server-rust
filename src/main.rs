use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    let start_line = http_request.first().unwrap();
    let path = start_line.split_whitespace().nth(1).unwrap();
    let response: String;
    if path == "/" {
        response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
    } else if path.starts_with("/echo/") {
        response = format!(
            "\
            HTTP/1.1 200 OK\r\n\
            Content-Type: text/plain\r\n\
            Content-Length: {0}\r\n\r\n\
            {1}",
            path.len() - 6,
            &path[6..]
        );
    } else {
        response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }
    stream.write_all(response.as_bytes()).unwrap();
}
