use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

/// A WebSocket echo server
fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").expect("Failed to bind");
    println!("WebSocket server listening on ws://127.0.0.1:9001");
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.expect("Failed to accept connection"))
                .expect("Failed to accept websocket connection");
            loop {
                let msg = match websocket.read() {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("Error reading message: {}", e);
                        break;
                    }
                };
                println!("Received: {}", msg);
                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    if let Err(e) = websocket.send(msg) {
                        eprintln!("Error writing message: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
