use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
};

pub fn start_server() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server running at http://localhost:8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Request failed: {e}");
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {e}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(&mut stream); // Read the HTTP request line.
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    println!("Request: {}", request_line.trim());

    if !request_line.starts_with("GET /download ") {
        write_response(&mut stream, "404 Not Found", "text/plain", b"Not Found")?;
        return Ok(());
    }
    let path = Path::new("./file.txt");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            write_response(
                &mut stream,
                "404 Not Found",
                "text/plain",
                b"File not found",
            )?;
            return Ok(());
        }
    };
    let size = file.metadata()?.len();

    // Send HTTP headers.
    write!(
        stream,
        "HTTP/1.1 200 OK\r\n\
        Content-Type: application/octet-stream\r\n\
        Content-Length: {}\r\n\
        Content-Disposition: attachment; filename=\"file.txt\"\r\n\
        Connection: close\r\n\
        \r\n",
        size
    )?;

    // Stream the file to the client.
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n])?;
    }
    stream.flush()?;
    Ok(())
}

fn write_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> io::Result<()> {
    println!("write_response()");
    write!(
        stream,
        "HTTP/1.1 {}\r\n\
        Content-Type: {}\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n",
        status,
        content_type,
        body.len()
    )?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}
