use std::{io::Error, net::UdpSocket};
use rdns_resolver_rs::lookup::handle_query;

fn main() -> Result<(), Error> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    loop {
        match handle_query(&socket) {
            Ok(_) => {},
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}