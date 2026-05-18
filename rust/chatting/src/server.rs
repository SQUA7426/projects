use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{process, time};

const TIMEOUT: u64 = 30;

#[derive(Debug, Clone)]
pub struct Server {
    ip: String,
    clients: Vec<u16>,
}

#[allow(dead_code)]
impl Server {
    pub fn init() -> Self {
        Self {
            ip: String::from(""),
            clients: vec![],
        }
    }

    pub fn new(ip: String) -> Self {
        Self {
            ip: ip,
            clients: vec![],
        }
    }

    pub fn write_into_buf(s: String) -> ([u8; 1024], usize) {
        let mut buf = [0; 1024];
        let sb = s.as_bytes();
        let sb_len = sb.len();

        buf[..sb_len].copy_from_slice(sb);
        (buf, sb_len)
    }

    pub fn echo_to_client(buf: &mut [u8], stream: &mut TcpStream) {
        std::str::from_utf8(&buf)
            .expect("NO DATA")
            .trim_matches(char::from(0))
            .trim()
            .to_string();
        let mut pos = 0;

        while pos < buf.len() {
            let w = stream.write(&buf).unwrap();
            pos += w;
        }
    }

    pub fn start(&mut self) {

        let listener = TcpListener::bind(&self.ip).expect("CREATE LISTENER");
        listener.set_nonblocking(true).expect("Set non block");

        let mut starttime: time::SystemTime = time::SystemTime::now();
        while starttime.elapsed().unwrap().as_secs() < TIMEOUT {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    self.clients.push(addr.port());

                    println!(
                        "| TCP-Connection accepted:{}{} |",
                        " ".repeat(22 - (addr.to_string().len())),
                        addr.to_string()
                    );

                    let (mut b, l) = Self::write_into_buf(addr.to_string());
                    Self::echo_to_client(&mut b[..l], &mut stream);

                    loop {
                        let rv: String = Self::receive(&self, &mut stream, starttime);

                        if !Self::handle_client(self, rv, &mut stream) {
                            break;
                        }
                    }
                }
                Err(_e) => {}
            }
        }
        println!("TIMEOUT_SHUTDOWN..\nClosing...");
    }

    pub fn receive(&self, stream: &mut TcpStream, mut t: time::SystemTime) -> String {
        loop {
            if t.elapsed().unwrap().as_secs() > TIMEOUT {
                println!(
                    "TCP-Connection to {} is shutting down...",
                    stream.peer_addr().unwrap()
                );
                return "TIMEOUT_SHUTDOWN".into();
            }
            let mut buf = [0; 1024];
            match stream.read(&mut buf) {
                Ok(0) => {
                    println!("Socket closed from client side!");
                    return "LOGOUT".into();
                }
                Ok(b) => {
                    let time_passed = t.elapsed().unwrap().as_millis();
                    // t = time::SystemTime::now();
                    let s = str::from_utf8(&buf[..b])
                        .expect("utf-8")
                        .trim_matches(char::from(0))
                        .trim()
                        .to_string();
                    if s.chars().any(|c| c == '$') {
                        return "LOGOUT".into();
                    }
                    println!("\nServer RV:\t{s}\tTime: {time_passed}");
                    return s;
                }
                Err(_e) => {
                    return "S_RECEIVE_ERR".into();
                }
            }
        }
    }
    pub fn handle_client(&mut self, rv: String, stream: &mut TcpStream) -> bool {
        match rv.as_str() {
            "SERVER_SHUTDOWN" => {
                if self.clients.len() == 0 {
                    println!("Closing..");
                    process::exit(0x0100);
                } else {
                    self.handle_client("LOGOUT".into(), stream);
                }
                false
            }
            "TIMEOUT_SHUTDOWN" => {
                println!("TIMEOUT Shutdown...\nClosing..");
                process::exit(0x0100)
            }
            "S_CLIENT_CLOSE" => {
                self.handle_client("SERVER_SHUTDOWN".into(), stream);
                false
            }
            "S_RECEIVE_ERR" => false,
            "LOGOUT" => {
                for (idx, client_port) in self.clients.clone().iter().enumerate() {
                    if *client_port == stream.peer_addr().expect("PEER_ADDR").port() {
                        println!("Client {client_port} logs out!\nRemoved from ClientList");
                        self.clients.remove(idx);
                        let (mut ex, lex) = Self::write_into_buf("LOGOUT".to_string());
                        println!("Server: SENDING LOGOUT...");
                        Self::echo_to_client(&mut ex[..lex], stream);
                    };
                }
                false
            }
            s => {
                let (mut sb, len) = Self::write_into_buf(s.to_string());
                Self::echo_to_client(&mut sb[..len], stream);
                true
            }
        }
    }
}
