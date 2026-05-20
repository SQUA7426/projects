use std::error::Error;
use std::fmt;
use std::io;
use std::result::Result;
use tokio::net::{TcpStream, UdpSocket};

use std::env;

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
        let mut stream = TcpStream::connect(format!("{}:{}", &self.ip, &self.tcp)).await?;

        stream.writable().await?;

        loop {
            match stream.try_write(format!("{}:{}", &self.ip, &self.udp).as_bytes()) {
                Ok(n) => {
                    break;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    async fn udp_peer(&self) -> io::Result<()> {
        let sock = UdpSocket::bind(format!("{}:{}", &self.ip, &self.udp)).await?;

        let mut buf = [0; 1024];
        loop {
            let (len, addr) = sock.recv_from(&mut buf).await?;
            println!(
                "Received {} from {:?}",
                str::from_utf8(&mut buf[..len])
                    .expect("Could not recv from")
                    .trim_matches(char::from(0))
                    .trim()
                    .to_string(),
                addr
            );

            let len = sock.send(&buf[..len]).await?;
            println!(
                "Send {}..",
                str::from_utf8(&mut buf[..len])
                    .expect("Could not recv from")
                    .trim_matches(char::from(0))
                    .trim()
                    .to_string()
            );
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

    // c.udp_peer().await?;
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
