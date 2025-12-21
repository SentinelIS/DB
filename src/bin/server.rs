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

    let mut buffer = [0; 4096]; // Increased buffer size for potentially larger queries

    // A connection now handles a single query and then closes.
    // This is a simpler request/response model that prevents the client from hanging
    // while waiting for a stream to end.
    match stream.read(&mut buffer) {
        Ok(size) => {
            // If size is 0, the client connected and disconnected without sending data.
            if size == 0 {
                println!("Client disconnected without sending data.");
                return;
            }

            // Convert the received bytes to a string, trimming whitespace.
            let input = String::from_utf8_lossy(&buffer[..size]).trim().to_string();

            // Process the command using the shared `execute_line` function.
            if !input.is_empty() {
                let result = execute_line(&input, &mut query_engine, &parser);
                
                // Send the raw result back to the client. `execute_line` handles formatting.
                if let Err(e) = stream.write_all(result.as_bytes()) {
                    eprintln!("Failed to write to stream: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("An error occurred reading from connection: {}", e);
        }
    }
    // The `stream` goes out of scope here, and the connection is automatically closed.
    // This is what signals EOF to the client's `read_to_end()` call.
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
