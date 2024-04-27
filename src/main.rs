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
        },
        "GET /cats HTTP/1.1" => ("HTTP/1.1 200 OK", "webpages/cats.html"),
        "GET /.well-known/pki-validation/398D27D43B2DE054E51F3817604CBCD5.txt HTTP/1.1" => 
        ("HTTP/1.1 200 OK","webpages/.well-known/pki-validation/398D27D43B2DE054E51F3817604CBCD5.txt"),

        _ => ("HTTP/1.1 404 NOT FOUND", "webpages/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
