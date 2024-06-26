pub mod request;
pub mod file;
pub mod response;

use std::{
    env,
    fs::{self, File},
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
                std::thread::spawn(|| handle_connection(_stream));
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

    let response = match path {
        "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        "/user-agent" => {
            let user_agent = http_request
                .iter()
                .find(|line| line.starts_with("User-Agent"))
                .unwrap();
            
            let message = user_agent.strip_prefix("User-Agent: ").unwrap();

            format!(
                "\
                HTTP/1.1 200 OK\r\n\
                Content-Type: text/plain\r\n\
                Content-Length: {}\r\n\r\n\
                {}",
                message.len(),
                message
            )
        }
        _ => {
            if path.starts_with("/echo/") {
                let message = path.strip_prefix("/echo/").unwrap();
                format!(
                    "\
                    HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: {}\r\n\r\n\
                    {}",
                    message.len(),
                    message
                )
            } else if path.starts_with("/files/") {
                let file_path = path.strip_prefix("/files/").unwrap();
                let cmd_line_args: Vec<String> = env::args().collect();
                let base_dir = cmd_line_args.get(2).unwrap();

                let file = File::open(format!("{}{}", base_dir, file_path));

                match file {
                    Ok(_) => {
                        let file_content = fs::read_to_string(format!("{}{}", base_dir, file_path))
                            .expect("Unable to read file");

                        format!(
                            "\
                            HTTP/1.1 200 OK\r\n\
                            Content-Type: application/octet-stream\r\n\
                            Content-Length: {}\r\n\r\n\
                            {}",
                            file_content.len(),
                            file_content
                        )
                    },
                    Err(_) => {
                        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                    }
                }
            } else {
                "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
}
