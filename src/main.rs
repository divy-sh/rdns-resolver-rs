mod byte_packet_buffer;
mod dns_packet;
mod dns_record;
mod query_type;
mod res_code;
mod dns_question;
mod dns_header;

use std::net::UdpSocket;
use byte_packet_buffer::BytePacketBuffer;
use dns_packet::DnsPacket;
use query_type::QueryType;
use dns_question::DnsQuestion;

fn main() -> Result<(), String> {
    let qname = "google.com";
    let qtype = QueryType::A;

    let server = ("8.8.8.8", 53);

    let socket = UdpSocket::bind(("0.0.0.0", 43210)).unwrap();

    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer)?;
    
    for i in 0..req_buffer.pos {
        print!("{:02X} ", req_buffer.buf[i]);
    }
    println!();

    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server).unwrap();

    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf).unwrap();

    let res_packet = DnsPacket::from_buffer(&mut res_buffer)?;
    println!("{:#?}", res_packet.header);

    for q in res_packet.questions {
        println!("{:#?}", q);
    }
    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }
    for rec in res_packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}