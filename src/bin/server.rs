// src/bin/server.rs

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// The server uses the same core logic from the library crate.
use rust_dbms::{
    parser::Parser,
    engine::QueryEngine,
    execute_line,
};

/// Handles a single client connection.
///
/// This function is spawned in a new thread for each incoming connection.
/// It creates a dedicated `QueryEngine` and `Parser` for the client,
/// ensuring that each client session is isolated.
///
/// # Arguments
/// * `stream` - A mutable `TcpStream` representing the client connection.
fn handle_client(mut stream: TcpStream) {
    // Each connection gets its own isolated database engine and parser.
    // This is crucial for preventing data races and ensuring session state
    // is not shared between concurrent users.
    let mut query_engine = QueryEngine::new();
    let parser = Parser::new();

    let mut buffer = [0; 1024]; // A buffer for reading incoming data.

    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                // If size is 0, the client has closed the connection.
                if size == 0 {
                    println!("Client disconnected.");
                    break;
                }

                // Convert the received bytes to a string, trimming whitespace.
                let input = String::from_utf8_lossy(&buffer[..size]).trim().to_string();

                // Process the command using the shared `execute_line` function.
                if !input.is_empty() {
                    let result = execute_line(&input, &mut query_engine, &parser);
                    
                    // Send the result back to the client. A newline is appended
                    // to help clients with line-based reading.
                    if let Err(e) = stream.write_all(format!("{}\n", result).as_bytes()) {
                        eprintln!("Failed to write to stream: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("An error occurred, terminating connection: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let address = "127.0.0.1:5555";
    let listener = TcpListener::bind(address).expect("Failed to bind to address");
    println!("Server listening on {}", address);

    // Accept incoming connections and spawn a new thread for each one.
    // This allows the server to handle multiple clients concurrently.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
