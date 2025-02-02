use rand::Rng;
use std::{
    net::{Ipv4Addr, UdpSocket},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    byte_packet_buffer::BytePacketBuffer, dns_packet::DnsPacket, dns_question::DnsQuestion,
    dns_record::DnsRecord, lru_cache::LRUCache, query_type::QueryType, res_code::ResultCode,
    utils::ROOT_NAME_SERVERS,
};

pub fn handle_queries(
    req_socket: &UdpSocket,
    query_socket: &UdpSocket,
    cache: Arc<Mutex<LRUCache>>,
) -> Result<(), String> {
    let mut req_buffer = BytePacketBuffer::default();
    loop {
        if let Ok((_, src)) = req_socket.recv_from(&mut req_buffer.buf) {
            // Spawn a new thread to handle the query
            let req_socket = req_socket.try_clone().unwrap();
            let query_socket = query_socket.try_clone().unwrap();
            let cache = Arc::clone(&cache);
            let mut req_buffer = req_buffer.clone();
            thread::spawn(move || {
                let mut res_buffer = BytePacketBuffer::default();
                let packet =
                    handle_query(&query_socket, &mut req_buffer, &mut cache.lock().unwrap())
                        .unwrap();
                if packet.write(&mut res_buffer).is_err() {
                    println!("Packet size overflow, truncated.");
                }
                let len = res_buffer.pos;
                let data = res_buffer.get_range(0, len).unwrap();
                req_socket.send_to(data, src).unwrap();
            });
        }
    }
}

pub fn handle_query(
    query_socket: &UdpSocket,
    req_buffer: &mut BytePacketBuffer,
    cache: &mut LRUCache,
) -> Result<DnsPacket, String> {
    let mut request = DnsPacket::from_buffer(req_buffer)?;
    let mut packet = DnsPacket::default();
    packet.header.id = request.header.id;
    packet.header.recursion_desired = true;
    packet.header.recursion_available = true;
    packet.header.response = true;
    packet.header.questions = 1;
    if let Some(question) = request.questions.pop() {
        println!("Received query: {:?}", question);
        match cache.get(&question.name) {
            Some(result) => {
                populate_dns_packet(&mut packet, question, &result);
            }
            None => {
                let root_name_server = &ROOT_NAME_SERVERS[0];
                if let Ok(result) = recursive_lookup(
                    query_socket,
                    &question.name,
                    question.qtype,
                    root_name_server.a,
                ) {
                    cache.put(&question.name, &result);
                    populate_dns_packet(&mut packet, question, &result);
                } else {
                    packet.header.rescode = ResultCode::SERVFAIL;
                }
            }
        }
    } else {
        packet.header.rescode = ResultCode::FORMERR;
    }

    Ok(packet)
}

fn recursive_lookup(
    query_socket: &UdpSocket,
    qname: &str,
    qtype: QueryType,
    mut ns: Ipv4Addr,
) -> Result<DnsPacket, String> {
    loop {
        println!("attempting lookup of {:?} {} with ns {}", qtype, qname, ns);
        let ns_copy = ns;
        let server = (ns_copy, 53);
        let response = lookup(query_socket, qname, qtype, server)?;
        if response.header.rescode == ResultCode::NXDOMAIN {
            return Ok(response);
        }
        if !response.answers.is_empty() && response.header.rescode == ResultCode::NOERROR {
            for answer in &response.answers {
                match answer {
                    DnsRecord::A { .. } | DnsRecord::AAAA { .. } => return Ok(response),
                    DnsRecord::CNAME { host, .. } => {
                        println!("CNAME found: Resolving {}", host);
                        return recursive_lookup(query_socket, host, qtype, ns);
                    }
                    _ => continue,
                }
            }
        }
        if let Some(new_ns) = response.get_resolved_ns(qname) {
            ns = new_ns;

            continue;
        }
        let new_ns_name = match response.get_unresolved_ns(qname) {
            Some(x) => x,
            None => return Ok(response),
        };
        let recursive_response = recursive_lookup(query_socket, new_ns_name, QueryType::A, ns)?;
        if let Some(new_ns) = recursive_response.get_random_a() {
            ns = new_ns;
        } else {
            return Ok(response);
        }
    }
}

fn lookup(
    query_socket: &UdpSocket,
    qname: &str,
    qtype: QueryType,
    server: (Ipv4Addr, u16),
) -> Result<DnsPacket, String> {
    let mut packet = DnsPacket::default();

    let mut rng = rand::thread_rng();
    packet.header.id = rng.gen::<u16>();
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::default();
    packet.write(&mut req_buffer)?;
    query_socket
        .send_to(&req_buffer.buf[0..req_buffer.pos], server)
        .unwrap();

    let mut res_buffer = BytePacketBuffer::default();
    query_socket.recv_from(&mut res_buffer.buf).unwrap();

    DnsPacket::from_buffer(&mut res_buffer)
}

fn populate_dns_packet(packet: &mut DnsPacket, question: DnsQuestion, result: &DnsPacket) {
    packet.questions.push(question);
    packet.header.rescode = result.header.rescode;
    packet.header.answers = result.answers.len() as u16;
    for rec in result.answers.iter() {
        println!("Answer: {:?}", rec);
        packet.answers.push(rec.clone());
    }
    for rec in result.authorities.iter() {
        println!("Authority: {:?}", rec);
        packet.authorities.push(rec.clone());
    }
    for rec in result.resources.iter() {
        println!("Resource: {:?}", rec);
        packet.resources.push(rec.clone());
    }
}
