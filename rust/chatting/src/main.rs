
#[path="./examples/client.rs"]
mod client;
#[path="./examples/server.rs"]
mod server;

fn main() {
    todo!();
}

#[cfg(test)]
mod test {
    #[test]
    fn create_test() {
        let _s = crate::server::Server::new("127.0.0.1:3000".into());
    }
}
