# Rust TCP Chat Application

A simple client-server chat application built in Rust using TCP sockets. This project allows multiple clients to connect to a server, set usernames, and exchange real-time text messages. It’s a beginner-friendly introduction to Rust’s networking, concurrency, and I/O capabilities.

## Features
- **Client-Server Architecture**: A server handles multiple client connections, broadcasting messages to all connected clients.
- **Usernames**: Clients can set custom usernames upon connecting, which are displayed in messages (e.g., `Alice: Hello, Bob!`).
- **Real-Time Messaging**: Messages are sent and received instantly over TCP.
- **Reliable Communication**: Uses TCP for guaranteed delivery and ordered messages.
- **Thread-Based Concurrency**: Each client runs in a separate thread, with thread-safe message broadcasting using `Arc<Mutex>`.

## Prerequisites
- **Rust**: Install Rust and Cargo using [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh