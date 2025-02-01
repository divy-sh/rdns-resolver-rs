use std::net::{Ipv4Addr, UdpSocket};
// use rand::{seq::SliceRandom, thread_rng};

use crate::{
    byte_packet_buffer::BytePacketBuffer, dns_packet::DnsPacket, dns_question::DnsQuestion, dns_record::DnsRecord, query_type::QueryType, res_code::ResultCode, root_name_servers::{RootNameServer, ROOT_NAME_SERVERS}
};

pub fn handle_query(socket: &UdpSocket) -> Result<(), String> {
    let mut req_buffer = BytePacketBuffer::new();

    let (_, src) = socket.recv_from(&mut req_buffer.buf).unwrap();

    let mut request = DnsPacket::from_buffer(&mut req_buffer)?;

    let mut packet = DnsPacket::new();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;
    packet.header.questions = 1;

    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);
        // currently choosing the 1st one as some of them are not working
        // let root_name_server = ROOT_NAME_SERVERS.choose(&mut thread_rng()).unwrap();
        let root_name_server = &ROOT_NAME_SERVERS[0];

        if let Ok(result) = recursive_lookup(&question.name, question.qtype, root_name_server) {
            packet.questions.push(question);
            packet.header.rescode = result.header.rescode;
            packet.header.answers = result.answers.len() as u16;

            for rec in result.answers {
                println!("Answer: {:?}", rec);
                packet.answers.push(rec);
            }
            for rec in result.authorities {
                println!("Authority: {:?}", rec);
                packet.authorities.push(rec);
            }
            for rec in result.resources {
                println!("Resource: {:?}", rec);
                packet.resources.push(rec);
            }
        } else {
            packet.header.rescode = ResultCode::SERVFAIL;
        }
    } else {
        packet.header.rescode = ResultCode::FORMERR;
    }

    let mut res_buffer = BytePacketBuffer::new();
    packet.write(&mut res_buffer)?;
    let len = res_buffer.pos;
    let data = res_buffer.get_range(0, len)?;
    socket.send_to(data, src).unwrap();

    Ok(())
}

fn recursive_lookup(qname: &str, qtype: QueryType, root_name_server: &RootNameServer) -> Result<DnsPacket, String> {
    let mut ns = root_name_server.a;

    loop {
        println!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);
        let ns_copy = ns;
        let server = (ns_copy, 53);
        let response = lookup(qname, qtype, server)?;

        if !response.answers.is_empty() && response.header.rescode == ResultCode::NOERROR {
            for answer in &response.answers {
                match answer {
                    DnsRecord::A { .. } | DnsRecord::AAAA { .. } => return Ok(response),
                    DnsRecord::CNAME { host, .. } => {
                        println!("CNAME found: Resolving {}", host);
                        return recursive_lookup(host, qtype, root_name_server);
                    }
                    _ => continue,
                }
            }
        }
        if response.header.rescode == ResultCode::NXDOMAIN {
            return Ok(response);
        }
        if let Some(new_ns) = response.get_resolved_ns(qname) {
            ns = new_ns;

            continue;
        }
        let new_ns_name = match response.get_unresolved_ns(qname) {
            Some(x) => x,
            None => return Ok(response),
        };
        let recursive_response = recursive_lookup(&new_ns_name, QueryType::A, root_name_server)?;
        if let Some(new_ns) = recursive_response.get_random_a() {
            ns = new_ns;
        } else {
            return Ok(response);
        }
    }
}

fn lookup(qname: &str, qtype: QueryType, server: (Ipv4Addr, u16)) -> Result<DnsPacket, String> {
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
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server).unwrap();

    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf).unwrap();

    DnsPacket::from_buffer(&mut res_buffer)
}