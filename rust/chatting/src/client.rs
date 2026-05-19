use regex::Regex;
use std::fmt::{Display, Formatter, Result, format};
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream, UdpSocket};
use std::time::Duration;
use std::{process, thread, time};

#[derive(Debug)]
pub struct Client {
    nickname: String,
    ip: String,
    tcp_port: i32,
    udp_port: i32,
}

#[allow(dead_code)]
impl Client {
    pub fn init() -> Self {
        Self {
            nickname: String::from("User123$"),
            ip: String::from("127.0.0.1"),
            tcp_port: 5000,
            udp_port: 0,
        }
    }

    pub fn new(nickname: String, ip: String, tcp_port: i32) -> Self {
        let reg = (Regex::new(r"^(\w{3,20}\$)$").unwrap(),
                    Regex::new(r"^(?:(?:25[0-5]|2[0-4]\d|1\d{2}|[1-9]\d|\d)\.){3}(?:25[0-5]|2[0-4]\d|1\d{2}|[1-9]\d|\d)$").unwrap());

        let check = (
            reg.0.is_match(&nickname),
            reg.1.is_match(&ip),
            tcp_port > 0 && tcp_port < 65535,
        );

        if !check.0 || !check.1 || !check.2 {
            println!("INVALID REGISTER: '{} {} {}' !!!", nickname, ip, tcp_port);
            return Self::init();
        }
        Self {
            nickname: nickname,
            ip: ip,
            tcp_port: tcp_port,
            udp_port: 0,
        }
    }

    pub fn client_tcp(&mut self) {
        let addr = format!("{}:{}", self.ip, self.tcp_port);
        let sock = TcpStream::connect(addr);
        // CONNECT

        let mut t = time::SystemTime::now();
        let mut outer_stream = if let Ok(mut stream) = sock {
            println!("Connected to Server!");

            let get_udp = Self::recv(&mut stream, t);
            self.udp_port = get_udp.parse::<i32>().unwrap();
            Some(stream)
        } else {
            println!("COULD NOT connect to Server!");
            None
        };

        thread::sleep(Duration::from_millis(20));
        // SEND

        while t.elapsed().unwrap().as_secs() < 10 {
            match outer_stream {
                Some(ref mut inner_stream) => {
                    let mut stream = inner_stream;
                    Self::send(&mut stream);
                    // RECV
                    let recv = Self::recv(&mut stream, t);

                    println!(
                        "\nClient RECV:\t{}\tTime: {}",
                        recv,
                        t.elapsed().unwrap().as_millis()
                    );

                    t = time::SystemTime::now();

                    if recv == "EXIT" {
                        println!("Client: EXIT -> SHUTDOWN BOTH... ");
                        stream.shutdown(Shutdown::Both).expect("Shutdown");
                        process::exit(0x0100);
                    } else if recv == "LOGOUT" {
                        println!("client {}: logging out..", self.udp_port);
                        outer_stream = None;
                        break;
                    } else if recv == "UDP" {
                        Self::client_udp(&self);
                    } else if recv == "C_SOCKED_CLOSED" {
                        break;
                    }
                }
                None => {}
            }
        }
    }

    pub fn send(stream: &mut TcpStream) {
        let mut msg = String::new();
        msg.clear();
        let _ = io::stdin().read_line(&mut msg).expect("String");
        let mut pos = 0;

        let str = String::from(msg.trim());
        if str.len() == 0 {
            Self::send(stream);
        } else {
            let byte_msg = str.as_bytes();

            while pos < byte_msg.len() {
                match stream.write(&byte_msg[pos..]) {
                    Ok(o) => pos += o,
                    Err(e) => panic!("client send error: {e:}"),
                }
            }
        }
    }

    pub fn recv(stream: &mut TcpStream, t: time::SystemTime) -> String {
        let mut b = [0; 1024];
        match stream.read(&mut b) {
            Ok(0) => {
                println!("Socket closed from Server side!");
                "C_SOCKET_CLOSED".into()
            }
            Ok(x) => {
                if t.elapsed().unwrap().as_secs() > 10 {
                    return "LOGOUT".into();
                }
                let s = str::from_utf8(&b[..x])
                    .expect("utf-8")
                    .trim_matches(char::from(0))
                    .trim();
                if s.chars().any(|c| c == '$') {
                    "LOGOUT".into()
                } else {
                    // ECHO
                    // println!("Data_LEN: {:?}\t\tDATA: {:?}", s.len(), s);
                    println!("MESSAGE:");
                    s.into()
                }
            }
            Err(_e) => "CLIENT_READ_ERR".into(),
        }
    }

    pub fn client_udp(&self) {
        let mut socket = UdpSocket::bind(format!("{}:{}", &self.ip, &self.udp_port));
        loop {
            match socket {
                Err(ref _e) => {
                    println!("UDP - SLEEP..");
                    thread::sleep(Duration::from_millis(20));
                    println!("UDP - SLEEP END..");
                }
                Ok(ref mut _o) => {
                    let s = _o;
                    println!("ENTER port to connect:");
                    let (mut buf, mut buffer) = ([0; 1024], [0; 1024]);
                    let in1 = io::stdin().read(&mut buf).expect("BUF NOT READ");
                    let bb = str::from_utf8(&buf).expect("REMOTE PORT").to_string();
                    let mut conn = if buf.len() < 7 {
                        s.connect(format!(
                            "{}:{}",
                            &self.ip,
                            bb
                        ))
                    } else {
                        s.connect(str::from_utf8(&buf).expect("REMOTE PORT").to_string())
                    };
                    match conn {
                        Err(_e) => {
                            let _tmp_sock = if buf.len() < 7 {
                                UdpSocket::bind(format!("{}:{}",&self.ip, bb)).expect("CONN ERR");
                                s.connect(format!(
                                    "{}:{}",
                                    &self.ip,
                                    bb
                                )).expect("REMOTE PORT");
                            } else {
                                UdpSocket::bind(format!("{}", bb)).expect("CONN ERR");
                                s.connect(str::from_utf8(&buf).expect("REMOTE PORT").to_string());
                            };
                        },
                        Ok(ref mut _o) => {
                            let mut so = _o;
                            println!("SOCK:\n{:#?}", so);
                            println!("MSG:");
                            let in2 = io::stdin().read(&mut buffer).expect("BUF NOT READ");
                            let (mut b, l) = Self::write_into_buf(in2.to_string());

                            println!("GOT SEND!");
                            println!("UDP - SLEEP..");
                            thread::sleep(Duration::from_millis(20));
                            println!("UDP - SLEEP END..");
                            buf = [0; 1024];
                            println!("UDP - RECV..");
                            let (amt, src) = so.recv_from(&mut buf).expect("UDP - RECEIVE ERR");
                            println!("UDP - RECV END..");
                            println!("\nReceiced: {} from {}\n", amt, src);
                        }
                    }
                }
            }
        }
    }

    pub fn write_into_buf(s: String) -> ([u8; 1024], usize) {
        let mut buf = [0; 1024];
        let sb = s.as_bytes();
        let sb_len = sb.len();

        buf[..sb_len].copy_from_slice(sb);
        (buf, sb_len)
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "nickname: {}, ip: {}, tcp_port: {}, udp_port: {}",
            self.nickname, self.ip, self.tcp_port, self.udp_port
        )
    }
}
