use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8080";

fn main() {
    let endpoint = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(endpoint).unwrap();

    for incoming_stream in listener.incoming() {
        let stream = incoming_stream.unwrap();
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    let request_first_line = request.lines().next().unwrap();
    let parts: Vec<&str> = request_first_line.split(" ").collect();
    let verb = parts[0];
    let path = get_file_path(parts[1]);

    println!("\nREQUEST:\n  verb: {}\n  path: {}", verb, parts[1]);

    let mime_type = get_file_mime_type(path.clone());
    let (response_content, size_content) = get_content(path.clone());

    let headers = [
        "HTTP/1.1 200 OK",
        &format!("Content-Length: {}", size_content) as &str,
        &format!("Content-Type: {}", mime_type) as &str,
        "\r\n",
    ];

    let mut response = headers.join("\r\n").to_string().into_bytes();
    response.extend(response_content);

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

fn get_file_mime_type(path: String) -> &'static str {
    let parts: Vec<&str> = path.split(".").collect();
    match parts.last().unwrap().clone() {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "png" => "image/png",
        "jpeg" | "jpg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "mp4" => "video/mp4",
        _ => "text/plain",
    }
}

fn get_file_path(path: &str) -> String {
    let path = if path == "/" { "/index.html" } else { path };
    format!("./static{}", path)
}

fn get_content(path: String) -> (Vec<u8>, usize) {
    let mut file = fs::File::open(path).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    (contents.clone(), contents.len())
}
