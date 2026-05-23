use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time;

use std::{env};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Server {
    ip: String,
    clients: Vec<String>,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            ip: addr,
            clients: vec![],
        }
    }

    pub fn get_ip(&self) -> String {
        self.ip.clone()
    }

    pub async fn add(&mut self, addr: String) -> Result<(), Box<dyn Error>> {
        self.clients.insert(self.clients.len(), addr);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ip: Vec<String> = env::args().collect();
    let mut serv = Arc::new(Mutex::new(Server::new(ip[1].clone())));
    let listener = TcpListener::bind(serv.lock().await.get_ip()).await?;
    println!("Listens...");

    loop {
        let (mut sock, _) = listener.accept().await?;

        let mut ser = Arc::clone(&serv);

        let thread = tokio::spawn(async move {
            println!("Accepted: {:#?}", sock);

            let mut buf = [0; 1024];

            loop {
                let n = match sock.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        let rv = str::from_utf8(&buf[..n])
                            .expect("SERVER RECV")
                            .trim_matches(char::from(0))
                            .trim()
                            .to_string();
                        if rv.chars().any(|c| c == '$') {
                            return;
                        }
                        println!("Server received: {}", rv);

                        // Arc::make_mut(&mut ser).add(rv).await.unwrap();
                        Arc::deref(&mut ser)
                            .lock()
                            .await
                            .add(rv)
                            .await
                            .expect("MUT ARC FAILED");
                        println!("After JOIN: {}", ser.lock().await);

                        let str: String = ser.lock().await.clone().to_string();
                        println!("Sending all ports to clients..");
                        match sock.write_all(str.as_bytes()).await {
                            Ok(_) => {
                                let _ = sock.flush().await;
                                println!("All sent!!!");
                                break;
                            },
                            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                                println!("Client disconnected prematurely (broken pipe), cleaning up connection...");
                            },
                            Err(e) => {
                                eprintln!("An unexpected Err: {}", e);
                            }
                        }
                        // println!("All sent!!!");
                        n
                    }
                    Err(e) => {
                        eprint!("Failed to read from socket; err = {e:?}");
                        return;
                    }
                };
                if let Err(e) = sock.write_all(&buf[0..n]).await {
                    eprint!("Failed to write into socket; err = {e:?}");
                    return;
                }
            }
        });
        // let _ = tokio::join!(thread);
    }
}
impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.clients.iter() {
            write!(f, "{} ", v)?;
        }
        Ok(())
    }
}
