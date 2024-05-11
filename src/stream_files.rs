// Purpose: Functions for streaming files to a client
use std::{
    fs,
    io::{self, Read, Write},
    net::TcpStream,
};

pub fn stream_text(stream: &mut TcpStream, status_line: &str, filename: &str, mime_type: String) {
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: max-age=180\r\n\r\n",
        status_line,
        mime_type,
        contents.len(),
    );
    println!("Response: {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.write(contents.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn stream_image(stream: &mut TcpStream, status_line: &str, filename: &str, mime_type: String) -> io::Result<()> {
    let metadata = fs::metadata(filename)?;
    let response = format!(
        "{}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: max-age=180\r\n\r\n",
        status_line,
        mime_type,
        metadata.len()
    );
    println!("Response: {}", response);
    stream.write_all(response.as_bytes())?;
    
    let mut file = fs::File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    stream.write(&buffer)?;
    stream.flush()?;
    Ok(())
}
