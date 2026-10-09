use crate::App;
use if_addrs::get_if_addrs;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

pub fn start_server(app: Arc<Mutex<App>>) -> Result<(), String> {
    let mut host: Option<String> = None;

    for interface in get_if_addrs().unwrap() {
        if let IpAddr::V4(ip) = interface.ip() {
            if !ip.is_loopback() {
                if interface.name == "wlo1" {
                    host = Some(ip.to_string());
                    break;
                }
            }
        }
    }

    let host = host.expect("unable to obtain host ip...");
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    app.lock().unwrap().host = Some(format!("http://{}:{}", host, port));
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let app_clonned = Arc::clone(&app);
                    if let Err(e) = handle_connection(app_clonned, stream) {
                        eprintln!("Request failed: {e}");
                    }
                }
                Err(e) => {
                    eprintln!("Connection failed: {e}");
                }
            }
        }
    });

    /*
    thread::spawn({
        let app = Arc::clone(&app);
        move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let app_cloned = Arc::clone(&app);

                        if let Err(e) = handle_connection(app_cloned, stream) {
                            eprintln!("Request failed: {e}");
                        }
                    }
                    Err(e) => {
                        eprintln!("Connection failed: {e}");
                    }
                }
            }
        }
    });
    */
    Ok(())
}

fn handle_connection(app: Arc<Mutex<App>>, mut stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(&mut stream); // Read the HTTP request line.
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    //println!("Request: {}", request_line.trim());

    let file_path = app.lock().unwrap().file_path.clone().unwrap();
    let file_name = app.lock().unwrap().file_name.clone().unwrap();
    let host = app.lock().unwrap().host.clone().unwrap();
    let link = app.lock().unwrap().link.clone().unwrap();
    let url = link.trim_start_matches(&host);

    if !request_line.starts_with(format!("GET {}", url).as_str()) {
        write_response(&mut stream, "404 Not Found", "text/plain", b"Not Found")?;
        return Ok(());
    }

    let path = Path::new(&file_path);
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
