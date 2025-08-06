use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    // Set to POST and target httpbin.org/post
    let request_type = "POST";
    let host = "httpbin.org";
    let path = "/post"; // Endpoint that accepts POST
    let port = 80;

    // Connect to the server
    let addr = format!("{}:{}", host, port);
    let mut stream = match TcpStream::connect(&addr) {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", addr, e);
            return;
        }
    };

    // Build the HTTP request
    let request = if request_type == "POST" {
        let body = "data=hello";
        format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            path, host, body.len(), body
        )
    } else {
        format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            path, host
        )
    };

    // Send the request
    if let Err(e) = stream.write_all(request.as_bytes()) {
        eprintln!("Failed to send request: {}", e);
        return;
    }
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush stream: {}", e);
        return;
    }

    // Read the response
    let mut response = String::new();
    if let Err(e) = stream.read_to_string(&mut response) {
        eprintln!("Failed to read response: {}", e);
        return;
    }

    // Print the response
    println!("Response:\n{}", response);
}