# Building a Robust Networked Key-Value Store in Rust

This document outlines the steps to build a robust networked key-value store in Rust.

## Project Setup

1.  **Create a New Project:**
    ```bash
    cargo new kvs
    cd kvs
    ```
2.  **Add Dependencies to `Cargo.toml`:**
    ```toml
    [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    tokio = { version = "1", features = ["full"] } # or async-std = "1"
    ```

## Core Data Structures and Operations (`src/lib.rs`)

3.  **Define Data Structures: DONE**
    * `DocType` (enum: `JSON`, `Raw`) 
    * `Document` (struct: `data: DocType`) 
    * `KeyValueStore` (struct: `store: HashMap<String, Document>`)
    * `KvsError` (enum: custom error types)

4.  **Implement `KeyValueStore` Methods: DONE**
    * `new()` (initialize store)
    * `get(&self, key: &str) -> Result<&Document, KvsError>`
    * `put(&mut self, key: String, value_str: String) -> Result<(), KvsError>` (with JSON validation)
    * `del(&mut self, key: &str)`

5.  **JSON Validation in `put()`: TODO**
    * Parse JSON using `serde_json::from_str`.
    * Perform custom validation (required fields, data types).
    * Return `KvsError::InvalidJson` or `KvsError::ValidationError` on failure.

6.  **Write Unit Tests:**
    * Test `put`, `get`, `del` operations.
    * Test JSON validation scenarios.
    * Verify error handling.

## Network Communication (`src/main.rs`)

7.  **Network Setup:**
    * Use `tokio::net::TcpListener` (or `async_std::net::TcpListener`).
    * Use `tokio::net::TcpStream` (or `async_std::net::TcpStream`).

8.  **Define Command Protocol:**
    * Text-based protocol (e.g., `GET <key>`, `SET <key> <value>`, `DELETE <key>`).
    * Parse commands from client input.

9.  **Asynchronous Client Handling:**
    * Use `tokio::spawn` (or `async_std::task::spawn`) for concurrent connections.
    * Implement `handle_client` function to process commands.

10. **Concurrency Control:**
    * Use `tokio::sync::Mutex` (or `async_std::sync::Mutex`) to protect `KeyValueStore`.
    * Acquire mutex before modifying the store.

11. **Network Error Handling:**
    * Handle network errors gracefully.
    * Send appropriate error messages to clients.

## Enhancements and Refinements

12. **Serialization/Deserialization (Optional):**
    * Use `serde` for custom data structures.
    * Consider binary formats (MessagePack).

13. **Persistence (Optional):**
    * File-based storage (append-only log).
    * Database (SQLite, Redis).

14. **Integration Tests (`tests/