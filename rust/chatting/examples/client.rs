use std::error::Error;
use std::fmt;
use std::result::Result;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::Mutex;

use std::env;
use std::sync::Arc;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Client {
    nickname: String,
    ip: String,
    tcp: String,
    udp: String,
}

impl Client {
    pub fn new(name: String, ip: String, tcp: String, udp: String) -> Self {
        Self {
            nickname: name,
            ip: ip,
            tcp: tcp,
            udp: udp,
        }
    }

    async fn connect_tcp_server(&self) -> Result<(), Box<dyn Error>> {
        let server_addr = format!("{}:{}", self.ip, self.tcp);
        let mut stream = TcpStream::connect(server_addr).await?;
        println!("Connected to TCP Server.");

        let msg = format!("{}:{}", self.ip, self.udp);
        stream.write_all(msg.as_bytes()).await?;

        let mut buf = [0; 1024];
        let n = stream.read(&mut buf).await?;

        let data = String::from_utf8_lossy(&buf[..n]).trim().to_string();
        println!("Client {} received: {}", self.udp, data);

        let my_addr = String::from(format!("{}:{}", self.ip, self.udp));

        let peers: Vec<String> = data
            .split_whitespace()
            .map(|s| s.to_string())
            .filter(|addr| addr != &my_addr)
            .collect();

        self.udp_peer(peers).await?;

        Ok(())
    }

    async fn udp_peer(&self, target_peers: Vec<String>) -> std::io::Result<()> {
        let my_addr = format!("{}:{}", self.ip, self.udp);
        let sock = Arc::new(tokio::net::UdpSocket::bind(&my_addr).await?);
        println!("UdpSocket bound to {}!", my_addr);
        println!("Type a message and press Enter to send to all peers.\n---");

        let shared_peers = Arc::new(Mutex::new(target_peers));

        let recv_sock = Arc::clone(&sock);
        let recv_peers = Arc::clone(&shared_peers);

        let listener_task = Self::recv(recv_sock, recv_peers).await;

        Self::send(sock, shared_peers).await;

        listener_task.abort();
        Ok(())
    }

    async fn recv(recv_sock: Arc<UdpSocket>, recv_peers: Arc<Mutex<Vec<String>>>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut stdout = tokio::io::stdout();

            loop {
                match recv_sock.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        let msg = String::from_utf8_lossy(&buf[..len]).trim().to_string();
                        let sender_addr = addr.to_string();

                        {
                            let mut peers_guard = recv_peers.lock().await;
                            if !peers_guard.contains(&sender_addr) {
                                peers_guard.push(sender_addr.clone());
                            }
                        }

                        let output = format!("\r\x1b[K[Received from {}]: {}\n> ", addr, msg);
                        let _ = stdout.write_all(output.as_bytes()).await;
                        let _ = stdout.flush().await;
                    }
                    Err(e) => {
                        eprintln!("UDP read error: {}", e);
                        break;
                    }
                }
            }
        })
    }

    async fn send(sock: Arc<UdpSocket>, shared_peers: Arc<Mutex<Vec<String>>>) {
        let mut stdin_reader = BufReader::new(tokio::io::stdin()).lines();
        let mut stdout = tokio::io::stdout();

        let _ = stdout.write_all(b"> ").await;
        let _ = stdout.flush().await;

        loop {
            if let Ok(Some(line)) = stdin_reader.next_line().await {
                let message = line.trim();
                if message.is_empty() {
                    let _ = stdout.write_all(b"> ").await;
                    let _ = stdout.flush().await;
                    continue;
                }

                let peers_guard = shared_peers.lock().await;
                for peer in peers_guard.iter() {
                    let _ = sock.send_to(message.as_bytes(), peer).await;
                }

                let echo = format!("\r\x1b[K[Broadcasted]: {}\n> ", message);
                let _ = stdout.write_all(echo.as_bytes()).await;
                let _ = stdout.flush().await;
            } else {
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let (n, ip, t, u) = (
        args[1].clone(),
        args[2].clone(),
        args[3].clone(),
        args[4].clone(),
    );

    let c = Client::new(n, ip, t, u);

    println!("{}", c.to_string());

    c.connect_tcp_server().await?;

    Ok(())
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Client: {} {}:{} {}:{}",
            self.nickname, self.ip, self.tcp, self.ip, self.udp
        )
    }
}
