mod byte_packet_buffer;
mod dns_packet;
mod dns_record;
mod query_type;
mod res_code;
mod dns_question;
mod dns_header;
mod lookup;

use std::{io::Error, net::UdpSocket};
use lookup::handle_query;

fn main() -> Result<(), Error> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;

    loop {
        match handle_query(&socket) {
            Ok(_) => {},
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}