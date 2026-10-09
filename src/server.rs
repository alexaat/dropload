use crate::App;
use if_addrs::get_if_addrs;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

pub fn start_server(app: Arc<Mutex<App>>) -> Result<(), String> {
    let mut host: Option<String> = None;

    for interface in get_if_addrs().map_err(|e| format!("cannot get ip addresses...\r\n{}", e))? {
        if let IpAddr::V4(ip) = interface.ip() {
            if !ip.is_loopback() {
                if interface.name == "wlo1" {
                    host = Some(ip.to_string());
                    break;
                }
            }
        }
    }

    let host = host.ok_or("unable to obtain host ip...")?;
    let listener = TcpListener::bind("0.0.0.0:0")
        .map_err(|e| format!("cannot bind tcp listener...\r\n{}", e))?;
    let port = listener
        .local_addr()
        .map_err(|e| format!("unable to obtain port...\r\n{}", e))?;
    let port = port.port();

    app.lock()
        .map_err(|e| format!("app data access error...\r\n{}", e))?
        .host = Some(format!("http://{}:{}", host, port));

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let app_clonned = Arc::clone(&app);
                    if let Err(e) = handle_connection(app_clonned, stream) {
                        eprintln!("{e}");
                    }
                }
                Err(e) => {
                    eprintln!("connection failed...\r\n{e}");
                }
            }
        }
    });
    Ok(())
}

fn handle_connection(app: Arc<Mutex<App>>, mut stream: TcpStream) -> Result<(), String> {
    let mut reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|e| format!("cannot read request...\r\n{}", e))?;

    let file_path = app
        .lock()
        .map_err(|e| format!("cannot get app data...\r\n{}", e))?
        .file_path
        .clone()
        .ok_or("file path is empty...")?;
    let file_name = app
        .lock()
        .map_err(|e| format!("cannot get app data...\r\n{}", e))?
        .file_name
        .clone()
        .ok_or("file name is empty")?;
    let host = app
        .lock()
        .map_err(|e| format!("cannot get app data...\r\n{}", e))?
        .host
        .clone()
        .ok_or("host is empty")?;
    let link = app
        .lock()
        .map_err(|e| format!("cannot get app data...\r\n{}", e))?
        .link
        .clone()
        .ok_or("host is empty")?;
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
    let size = file
        .metadata()
        .map_err(|e| format!("cannot get file metadata...\r\n{}", e))?
        .len();

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
    )
    .map_err(|e| format!("cannot write response...\r\n{}", e))?;

    // Stream the file to the client.
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("cannot read file...\r\n{}", e))?;
        if n == 0 {
            break;
        }
        stream
            .write_all(&buffer[..n])
            .map_err(|e| format!("cannot write response...\r\n{}", e))?;
    }
    stream
        .flush()
        .map_err(|e| format!("cannot flush response...\r\n{}", e))?;
    Ok(())
}

fn write_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> Result<(), String> {
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
    )
    .map_err(|e| format!("cannot write response...\r\n{}", e))?;
    stream
        .write_all(body)
        .map_err(|e| format!("cannot write response...\r\n{}", e))?;
    stream
        .flush()
        .map_err(|e| format!("cannot flush response...\r\n{}", e))?;
    Ok(())
}
