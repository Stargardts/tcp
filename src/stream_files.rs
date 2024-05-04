// Purpose: Functions for streaming files to a client
use std::{
    fs,
    io::prelude::*,
    net::TcpStream,
};

pub fn stream_text(stream: &mut TcpStream, status_line: &str, filename: &str, mime_type: String) {
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        mime_type,
        contents.len(),
    );
    println!("Response: {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.write(contents.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn stream_image (stream: &mut TcpStream, status_line: &str, filename: &str, mime_type: String) {
    let contents = fs::read(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        mime_type,
        contents.len()
    );
    println!("Response: {}", response);
    stream.write(response.as_bytes()).unwrap();
    let mut buf = vec![0; 1024];
    let mut file = fs::File::open(filename).unwrap();
    loop {
        let n = file.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        stream.write(&buf[..n]).unwrap();
    }
    stream.flush().unwrap();
}
