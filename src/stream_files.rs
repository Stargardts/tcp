// Purpose: Functions for streaming files to a client
use std::{
    fs,
    io::prelude::*,
    net::TcpStream,
};

pub fn stream_text(stream: &mut TcpStream, status_line: &str, filename: &str, mime_type: String) {
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        mime_type,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
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
    stream.write(response.as_bytes()).unwrap();
    stream.write(&contents).unwrap();
    stream.flush().unwrap();
}
