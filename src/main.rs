use std::{io::Error, net::UdpSocket, sync::{Arc, Mutex}};
use rdns_resolver_rs::{lookup::handle_queries, lru_cache::LRUCache};

fn main() -> Result<(), Error> {
    let req_socket = UdpSocket::bind(("0.0.0.0", 2053)).unwrap();
    let query_socket = UdpSocket::bind(("0.0.0.0", 43210)).unwrap();
    let cache = Arc::new(Mutex::new(LRUCache::new(100_000)));  // Shared cache for all threads
    loop {
        match handle_queries(&req_socket, &query_socket, cache.clone()) {
            Ok(_) => {},
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}