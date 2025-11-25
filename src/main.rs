mod storage;
mod wal;
mod parser;
mod engine;

use std::io::{self, Write};
use storage::StorageEngine;
use wal::WriteAheadLog;
use parser::{Command, Parser};

fn main() {
    println!("IsentaDB v0.1.0");
    println!("Type 'help' for commands, 'exit' to quit\n");

    // Initialize storage and WAL
    let _storage = StorageEngine::new("data.db");
    let _wal = WriteAheadLog::new("data.wal");
    let mut query_engine = engine::QueryEngine::new();
    let parser = Parser::new();

    // Simple REPL loop
    loop {
        print!("isenta> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Error reading input");
                continue;
            }
        }

        let input = input.trim();

        // Skip empty input
        if input.is_empty() {
            continue;
        }

        // Handle special commands
        match input.to_lowercase().as_str() {
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "help" => {
                print_help();
                continue;
            }
            _ => {}
        }

        // Parse and execute SQL command
        let command = parser.parse(input);
        match command {
            Command::CreateTable { name, columns } => {
                match query_engine.execute_create_table(name.clone(), columns.clone()) {
                    Ok(_) => println!("Table '{}' created successfully", name),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Insert { table, values } => {
                match query_engine.execute_insert(table.clone(), values.clone()) {
                    Ok(_) => println!("Inserted {} row(s) into '{}'", 1, table),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Select { table, columns: _ } => {
                match query_engine.execute_select(table.clone()) {
                    Ok(rows) => {
                        if rows.is_empty() {
                            println!("No rows found");
                        } else {
                            // Print header
                            if let Some(schema) = query_engine.get_table_schema(&table) {
                                let header: Vec<String> = schema
                                    .columns
                                    .iter()
                                    .map(|c| format!("{} ({})", c.name, c.data_type))
                                    .collect();
                                println!("{}", header.join(" | "));
                                println!("{}", "-".repeat(header.join(" | ").len()));

                                // Print rows
                                for row in rows {
                                    println!("{}", row.values.join(" | "));
                                }
                            }
                        }
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            Command::Unknown(cmd) => {
                println!("Unknown command: {}", cmd);
                println!("Type 'help' for available commands");
            }
        }
    }
}

fn print_help() {
    println!("\nAvailable commands:");
    println!("  CREATE TABLE <name> (<columns>)");
    println!("  INSERT INTO <table> VALUES (<values>)");
    println!("  SELECT * FROM <table>");
    println!("  help  - Show this help message");
    println!("  exit  - Quit the database\n");
}

