use rdns_resolver_rs::{lookup::handle_queries, lru_cache::LRUCache, utils};
use std::{
    io::Error,
    net::UdpSocket,
    sync::{Arc, Mutex},
};

fn main() -> Result<(), Error> {
    let req_socket = UdpSocket::bind((utils::LOCAL_HOST, utils::REQ_PORT)).unwrap();
    let query_socket = UdpSocket::bind((utils::LOCAL_HOST, utils::QUERY_PORT)).unwrap();
    let cache = Arc::new(Mutex::new(LRUCache::new(100_000))); // Shared cache for all threads
    loop {
        match handle_queries(&req_socket, &query_socket, cache.clone()) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
