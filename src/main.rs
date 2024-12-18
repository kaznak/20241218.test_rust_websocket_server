use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept_hdr;
use tungstenite::handshake::server::{Request, Response};

/// A WebSocket echo server
fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").expect("Failed to bind");
    println!("WebSocket server listening on ws://127.0.0.1:9001");
    for stream in server.incoming() {
        let stream = stream.expect("Failed to accept connection");
        spawn(move || {
            // HTTPヘッダやURI等を出力した上でWebSocketにアップグレードするためのコールバック
            let callback = |req: &Request, mut response: Response| {
                println!("Received HTTP request before WebSocket upgrade:");
                println!("Method: {}", req.method());
                println!("URI: {}", req.uri());
                println!("Version: {:?}", req.version());
                println!("Headers:");
                for (header_name, header_value) in req.headers() {
                    println!(
                        "  {}: {}",
                        header_name,
                        header_value.to_str().unwrap_or("<non-UTF-8>")
                    );
                }

                // 必要であればレスポンス側に追加ヘッダを入れることも可能
                response
                    .headers_mut()
                    .insert("X-Custom-Header", "MyValue".parse().unwrap());

                Ok(response)
            };

            // WebSocket 接続の確立（HTTPヘッダ出力後にアップグレード）
            let mut websocket = match accept_hdr(stream, callback) {
                Ok(ws) => ws,
                Err(e) => {
                    eprintln!("Failed to accept websocket connection: {}", e);
                    return;
                }
            };

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
