use rdns_resolver_rs::dns_packet::DnsPacket;
use rdns_resolver_rs::byte_packet_buffer::BytePacketBuffer;
use rdns_resolver_rs::dns_header::DnsHeader;
use rdns_resolver_rs::dns_question::DnsQuestion;
use rdns_resolver_rs::dns_record::DnsRecord;
use rdns_resolver_rs::query_type::QueryType;
use std::net::Ipv4Addr;

#[test]
fn test_dns_packet_creation() {
    let packet = DnsPacket::new();
    assert_eq!(packet.questions.len(), 0);
    assert_eq!(packet.answers.len(), 0);
    assert_eq!(packet.authorities.len(), 0);
    assert_eq!(packet.resources.len(), 0);
}

#[test]
fn test_dns_packet_from_buffer() {
    let mut buffer = BytePacketBuffer::new();
    // Simulate writing a header to buffer
    let mut header = DnsHeader::new();
    header.questions = 1;
    header.write(&mut buffer).unwrap();

    // Simulate writing a question to buffer
    let question = DnsQuestion::new("example.com".to_string(), QueryType::A);
    question.write(&mut buffer).unwrap();

    buffer.seek(0).unwrap(); // Reset buffer position

    let packet = DnsPacket::from_buffer(&mut buffer).unwrap();
    assert_eq!(packet.header.questions, 1);
    assert_eq!(packet.questions.len(), 1);
    assert_eq!(packet.questions[0].name, "example.com");
}

#[test]
fn test_dns_packet_write() {
    let mut buffer = BytePacketBuffer::new();
    let mut packet = DnsPacket::new();

    packet.header.id = 1234;
    packet.questions.push(DnsQuestion::new("example.com".to_string(), QueryType::A));

    packet.write(&mut buffer).unwrap();
    assert!(buffer.pos > 0); // Ensure data was written
}

#[test]
fn test_get_random_a() {
    let mut packet = DnsPacket::new();
    packet.answers.push(DnsRecord::A {
        domain: "example.com".to_string(),
        addr: Ipv4Addr::new(192, 168, 1, 1),
        ttl: 60,
    });

    let result = packet.get_random_a();
    assert_eq!(result, Some(Ipv4Addr::new(192, 168, 1, 1)));
}

#[test]
fn test_get_resolved_ns() {
    let mut packet = DnsPacket::new();
    packet.authorities.push(DnsRecord::NS {
        domain: "example.com".to_string(),
        host: "ns1.example.com".to_string(),
        ttl: 60,
    });
    packet.resources.push(DnsRecord::A {
        domain: "ns1.example.com".to_string(),
        addr: Ipv4Addr::new(8, 8, 8, 8),
        ttl: 60,
    });

    let result = packet.get_resolved_ns("example.com");
    assert_eq!(result, Some(Ipv4Addr::new(8, 8, 8, 8)));
}

#[test]
fn test_get_unresolved_ns() {
    let mut packet = DnsPacket::new();
    packet.authorities.push(DnsRecord::NS {
        domain: "example.com".to_string(),
        host: "ns1.example.com".to_string(),
        ttl: 60,
    });

    let result = packet.get_unresolved_ns("example.com");
    assert_eq!(result, Some("ns1.example.com"));
}