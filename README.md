# Networked Key-Value Store in Rust

This project implements a networked key-value store (KVS) in **Rust** using **Tokio**. The server allows clients to store, retrieve, and delete key-value pairs over a network.

## Features

- **KVS operations**: Set, get, and delete key-value pairs.
- **Networked**: TCP server with asynchronous handling using Tokio.
- **Concurrency**: Handles multiple clients efficiently.

## TODO
- add json document support
- cleanup
- finish ping pong 

## Usage
```cargo run -- <server | client> <ip | localhost:8080>```
After you setup a local server/client setup, you can interact with the KVS prompt engine on the client side using the following commands
- ```SET <KEY> <String>```
- ```GET <KEY>```
- ```DEL <KEY>```


## Building/Requirements
Build like a regular rust project using ```cargo```