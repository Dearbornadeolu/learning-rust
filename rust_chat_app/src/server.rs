use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;

// Store client information (stream and username)
struct Client {
    stream: TcpStream,
    username: String,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    let (tx, rx): (Sender<String>, Receiver<String>) = channel();
    let clients = Arc::new(Mutex::new(Vec::<Client>::new()));
    let clients_for_broadcast = Arc::clone(&clients);
    
    thread::spawn(move || {
        broadcast_messages(rx, clients_for_broadcast);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let tx = tx.clone();
                let clients = Arc::clone(&clients);
                thread::spawn(|| {
                    handle_client(stream, tx, clients);
                });
            }
            Err(e) => {
                eprintln!("Error accepting client: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, tx: Sender<String>, clients: Arc<Mutex<Vec<Client>>>) {
    let mut buffer = [0; 1024];
    let client_addr = stream.peer_addr().unwrap().to_string();
    println!("New client connected: {}", client_addr);

    // Read the username first
    let username = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            let message = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
            if message.starts_with("USERNAME:") {
                message.strip_prefix("USERNAME:").unwrap_or("Anonymous").to_string()
            } else {
                "Anonymous".to_string()
            }
        }
        _ => {
            eprintln!("Failed to read username from client {}", client_addr);
            "Anonymous".to_string()
        }
    };

    println!("Client {} assigned username: {}", client_addr, username);

    // Add client to shared list
    {
        let mut clients = clients.lock().expect("Failed to lock clients");
        clients.push(Client {
            stream: stream.try_clone().expect("Failed to clone stream"),
            username: username.clone(),
        });
    }

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client {} ({}) disconnected", client_addr, username);
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                if !message.is_empty() {
                    let formatted_message = format!("{}: {}", username, message);
                    println!("Received: {}", formatted_message);
                    if tx.send(formatted_message).is_err() {
                        eprintln!("Failed to send message to broadcast channel");
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client {} ({}): {}", client_addr, username, e);
                break;
            }
        }
    }

    // Remove client on disconnect
    {
        let mut clients = clients.lock().expect("Failed to lock clients");
        clients.retain(|client| client.username != username);
    }
}

fn broadcast_messages(rx: Receiver<String>, clients: Arc<Mutex<Vec<Client>>>) {
    loop {
        match rx.recv() {
            Ok(message) => {
                println!("Broadcasting: {}", message);
                let mut clients = clients.lock().expect("Failed to lock clients");
                clients.retain_mut(|client| {
                    match client.stream.write_all(format!("{}\n", message).as_bytes()) {
                        Ok(_) => client.stream.flush().is_ok(),
                        Err(_) => false,
                    }
                });
            }
            Err(e) => {
                eprintln!("Broadcast error: {}", e);
                break;
            }
        }
    }
}