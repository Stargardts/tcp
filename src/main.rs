use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use tcp::ThreadPool;
use mime_guess::from_path;
mod files;
use files::FileRequest;
mod stream_files;
use stream_files::{stream_text, stream_image};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to port 8080");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {}", request_line);

    let (status_line, filename) = match &request_line[..].split(" ").collect::<Vec<&str>>()[..] {
        ["GET", path, _] => {
            let file_request = FileRequest::from_path(path);
            match file_request {
                Some(file_request) => {
                    let filename = file_request.get_filename();
                    ("HTTP/1.1 200 OK", filename)
                }
                None => ("HTTP/1.1 404 NOT FOUND", "webpages/404.html"),
            }
        }
        _ => ("HTTP/1.1 400 BAD REQUEST", "webpages/400.html"),
    };

    let mime_type = from_path(filename).first_or_octet_stream().to_string();

    if mime_type.starts_with("text/") {
        stream_text(&mut stream, status_line, filename, mime_type);
    }
    else if mime_type.starts_with("image/") {
        stream_image(&mut stream, status_line, filename, mime_type).unwrap();
    }
}
