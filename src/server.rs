use if_addrs::get_if_addrs;
use std::net::IpAddr;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

const HOST: &'static str = "http://127.0.0.1";

pub fn start_server() -> String {
    let mut host: Option<String> = None;

    for interface in get_if_addrs().unwrap() {
        if let IpAddr::V4(ip) = interface.ip() {
            if !ip.is_loopback() {
                println!("{}: {}", interface.name, ip);
                if interface.name == "wlo1" {
                    host = Some(ip.to_string());
                }
            }
        }
    }

    let host = host.expect("unable to obtain host ip...");
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("Server running at {}:{}", host, port);

    let file_path = "/home/bocal/Downloads/alice.jpeg".to_string();
    let file_name = "alice.jpeg".to_string();

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = handle_connection(stream, file_name.clone(), file_path.clone())
                    {
                        eprintln!("Request failed: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("Connection failed: {e}");
                }
            }
        }
    });
    format!("http://{}:{}", host, port)
}

fn handle_connection(
    mut stream: TcpStream,
    file_name: String,
    file_path: String,
) -> io::Result<()> {
    let mut reader = BufReader::new(&mut stream); // Read the HTTP request line.
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    println!("Request: {}", request_line.trim());

    // if !request_line.starts_with("GET /download ") {
    //     write_response(&mut stream, "404 Not Found", "text/plain", b"Not Found")?;
    //     return Ok(());
    // }

    let path = Path::new(&file_path);
    //let link = app.link.as_ref().unwrap();
    //let path = Path::new(link);
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
        Content-Disposition: attachment; filename=\"{}\"\r\n\
        Connection: close\r\n\
        \r\n",
        size, file_name
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
