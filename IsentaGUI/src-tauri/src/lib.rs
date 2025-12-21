use std::io::{Read, Write};
use std::net::TcpStream;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Executes a query by sending it to the TCP server and returning the response.
#[tauri::command]
fn execute_query(query: &str) -> Result<String, String> {
    // Establish a new connection to the IsentaDB server for each query.
    match TcpStream::connect("127.0.0.1:5555") {
        Ok(mut stream) => {
            // Send the query string to the server.
            if let Err(e) = stream.write_all(query.as_bytes()) {
                return Err(format!("Failed to send query to server: {}", e));
            }
            
            // Shutdown the write half of the stream to signal the end of the request.
            // This is important for the server to know that we are done sending data.
            if let Err(e) = stream.shutdown(std::net::Shutdown::Write) {
                return Err(format!("Failed to signal end of query: {}", e));
            }

            // Read the entire response from the server until the stream is closed.
            let mut buffer = Vec::new();
            match stream.read_to_end(&mut buffer) {
                Ok(_) => {
                    // Convert the response bytes to a UTF-8 string.
                    let response = String::from_utf8_lossy(&buffer).to_string();
                    Ok(response)
                }
                Err(e) => Err(format!("Failed to read response from server: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to connect to IsentaDB server at 127.0.0.1:5555. Is it running? Error: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, execute_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
