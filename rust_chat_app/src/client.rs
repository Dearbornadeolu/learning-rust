use std::io::{self, Read, Write, stdout};
use std::net::TcpStream;
use std::thread;

fn main() {
    // Prompt for username
    println!("Enter your username:");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");
    let mut username = username.trim().to_string(); // Make this mutable
    if username.is_empty() {
        println!("Username cannot be empty. Using 'Anonymous'.");
        username.push_str("Anonymous");
    }

    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
    println!("Connected to server at 127.0.0.1:8080");

    // Send username to server
    stream
        .write_all(format!("USERNAME:{}\n", username).as_bytes())
        .expect("Failed to send username");
    stream.flush().expect("Failed to flush stream");

    let mut read_stream = stream.try_clone().expect("Failed to clone stream");

    thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match read_stream.read(&mut buffer) {
                Ok(0) => {
                    println!("Server disconnected");
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                    println!("{}", message);
                    stdout().flush().expect("Failed to flush stdout");
                }
                Err(e) => {
                    eprintln!("Error reading from server: {}", e);
                    break;
                }
            }
        }
    });

    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let message = input.trim();
        if message == "exit" {
            break;
        }
        if !message.is_empty() {
            if stream.write_all(format!("{}\n", message).as_bytes()).is_err() {
                eprintln!("Failed to send message to server");
                break;
            }
            stream.flush().expect("Failed to flush stream");
        }
    }
}