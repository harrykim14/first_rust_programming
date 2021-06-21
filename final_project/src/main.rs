use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use std::fs;

fn main() {
    // 127.0.0.1:7878 주소로 TCP 연결을 대기하기
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // incoming 메서드는 연속된 스트림에 대한 반복자를 리턴한다
    for stream in listener.incoming() {
        // unwrap 메서드를 호출해서 스트림에 문제가 있으면 프로그램을 중단하도록 하기
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // println!("요청: {}", String::from_utf8_lossy(&buffer[..]));

    /*
    let get = b"GET / HTTP/`.`\r\n";
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("{}{}", status_line, contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    */
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string("404.html").unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
