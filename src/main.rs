use std::{io::Error, net::UdpSocket};
use rdns_resolver_rs::{lookup::handle_query, lru_cache::LRUCache};

fn main() -> Result<(), Error> {
    let req_socket = UdpSocket::bind(("0.0.0.0", 2053))?;
    let query_socket = UdpSocket::bind(("0.0.0.0", 43210)).unwrap();

    let mut cache = LRUCache::new(100_000);
    loop {
        match handle_query(&req_socket, &query_socket, &mut cache) {
            Ok(_) => {},
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}