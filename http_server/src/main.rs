use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream);
    }
}

fn handle_request(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let _http_req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    send_res(stream);
    // println!("Request: {http_req:#?}");
}

fn send_res(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let content_type = "Content-Type: text/html";
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n{content_type}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
