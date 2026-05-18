mod client;
mod server;

use crate::client::Client;
use crate::server::Server;

use std::thread;

#[warn(unused, unused_variables)]
fn main() {
    let ip = String::from("127.0.0.1:5000");
    let peer_server = Server::new(ip);


    let mut c = Client::new(String::from("Thomas123$"), String::from("127.0.0.1"), 5000);
    let mut c2 = Client::init();
    let mut c3 = Client::init();
    let mut c4 = Client::init();

    println!("");

    let mut cloned_server = peer_server.clone();

    let server_thread = thread::spawn(move || cloned_server.start());

    let client_thread = thread::spawn(move || c.client_tcp());
    let client_thread2 = thread::spawn(move || c2.client_tcp());
    let client_thread3 = thread::spawn(move || c3.client_tcp());
    let client_thread4 = thread::spawn(move || c4.client_tcp());

    client_thread.join().unwrap();
    client_thread2.join().unwrap();
    client_thread3.join().unwrap();
    client_thread4.join().unwrap();

    server_thread.join().unwrap();
}

#[cfg(test)]
mod test {

    use crate::{Client};

    #[test]
    fn client_new() {
        let _c1 = Client::init();
        let _c2 = Client::new(String::from("Tom123$"), String::from("255.0.0.1"), 1234);
    }
}
