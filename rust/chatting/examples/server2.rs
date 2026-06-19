use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::env;

#[derive(Debug, Clone)]
struct Client {
    nick: String,
    ip: String,
    port: u16,
}

#[derive(Debug)]
struct Server {
    clients: Vec<Client>,
}

impl Server {
    fn new() -> Self {
        Self {
            clients: Vec::new(),
        }
    }

    fn add(&mut self, c: Client) {
        self.clients.push(c);
    }

    fn snapshot(&self) -> Vec<Client> {
        self.clients.clone()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = args.get(1).expect("Usage: server <IP:PORT>");

    let state = Arc::new(Mutex::new(Server::new()));
    let listener = TcpListener::bind(addr).await?;

    println!("Server listening on {}", addr);

    loop {
        let (socket, peer) = listener.accept().await?;
        println!("Client connected: {}", peer);

        let state = Arc::clone(&state);

        tokio::spawn(async move {
            let mut reader = BufReader::new(socket);
            let mut line = String::new();

            // =========================
            // 1. REGISTER PHASE
            // =========================
            let n = match reader.read_line(&mut line).await {
                Ok(n) => n,
                Err(_) => return,
            };

            if n == 0 {
                return;
            }

            let msg = line.trim().to_string();

            if !msg.starts_with("REGISTER") {
                return;
            }

            let parts: Vec<&str> = msg.split('|').collect();
            if parts.len() < 4 {
                return;
            }

            let client = Client {
                nick: parts[1].to_string(),
                ip: parts[2].to_string(),
                port: parts[3].parse().unwrap_or(0),
            };

            {
                let mut srv = state.lock().await;
                srv.add(client);
            }

            // send REGISTER_OK
            let socket = reader.into_inner();
            let mut socket = socket;

            let _ = socket.write_all(b"REGISTER_OK|0\n").await;

            // send USER LIST snapshot
            let snapshot = {
                let srv = state.lock().await;
                srv.snapshot()
            };

            let _ = socket
                .write_all(format!("USER_LIST_BEGIN|{}\n", snapshot.len()).as_bytes())
                .await;

            for c in snapshot {
                let line = format!(
                    "USER_ITEM|{}|{}|{}\n",
                    c.nick, c.ip, c.port
                );
                let _ = socket.write_all(line.as_bytes()).await;
            }

            let _ = socket.write_all(b"USER_LIST_END\n").await;

            // =========================
            // 2. PERSISTENT SESSION LOOP
            // =========================
            let mut reader = BufReader::new(socket);
            let mut line = String::new();

            loop {
                line.clear();

                let n = match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("Client disconnected");
                        return;
                    }
                    Ok(n) => n,
                    Err(_) => return,
                };

                let msg = line.trim().to_string();
                println!("Received: {}", msg);

                // -------------------------
                // PING / keepalive example
                // -------------------------
                if msg.starts_with("PING") {
                    let _ = reader.get_mut().write_all(b"PONG\n").await;
                    continue;
                }

                // -------------------------
                // optional broadcast handling placeholder
                // -------------------------
                if msg.starts_with("BROADCAST") {
                    // hier könntest du später alle Clients senden
                    let _ = reader.get_mut().write_all(b"BROADCAST_OK\n").await;
                    continue;
                }

                // -------------------------
                // optional disconnect
                // -------------------------
                if msg.contains("$") {
                    println!("Client requested disconnect");
                    return;
                }

                // echo fallback (debug)
                let response = format!("ECHO|{}\n", msg);
                let _ = reader.get_mut().write_all(response.as_bytes()).await;
            }
        });
    }
}
