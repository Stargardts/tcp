use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use tcp::ThreadPool;

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

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "webpages/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "webpages/index.html")
        }
        "GET /cats HTTP/1.1" => ("HTTP/1.1 200 OK", "webpages/cats.html"),
        "GET /images/pic.jpg HTTP/1.1" => ("HTTP/1.1 200 OK", "webpages/images/pic.jpg"),
        "GET /styles/style.css HTTP/1.1" => ("HTTP/1.1 200 OK", "webpages/styles/style.css"),
        _ => ("HTTP/1.1 404 NOT FOUND", "webpages/404.html"),
    };

    if filename.ends_with(".jpg") {
        let file = fs::File::open(filename).unwrap();
        let mime_type = "image/webp".to_string();
        let metadata = file.metadata().unwrap();
        let response = format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            status_line,
            mime_type,
            metadata.len()
        );

        stream.write_all(response.as_bytes()).unwrap();

        let mut buf_reader = BufReader::new(file);
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = buf_reader.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            stream.write_all(&buffer[..bytes_read]).unwrap();
        }

        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string(filename).unwrap();
        let mime_type = mime_guess::from_path(filename)
            .first_or_octet_stream()
            .to_string();

        let response = format!(
            "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            mime_type,
            contents.len(),
            contents
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
