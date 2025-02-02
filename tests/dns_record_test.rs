use rdns_resolver_rs::{byte_packet_buffer::BytePacketBuffer, dns_record::DnsRecord};
use std::net::{Ipv4Addr, Ipv6Addr};

#[test]
fn test_dns_record_a() {
    let mut buffer = BytePacketBuffer::default();
    let record = DnsRecord::A {
        domain: "example.com".to_string(),
        addr: Ipv4Addr::new(127, 0, 0, 1),
        ttl: 3600,
    };
    record.write(&mut buffer).unwrap();

    buffer.seek(0).unwrap();
    let parsed_record = DnsRecord::read(&mut buffer).unwrap();

    assert_eq!(record, parsed_record);
}

#[test]
fn test_dns_record_aaaa() {
    let mut buffer = BytePacketBuffer::default();
    let record = DnsRecord::AAAA {
        domain: "example.com".to_string(),
        addr: Ipv6Addr::new(0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 1),
        ttl: 3600,
    };
    record.write(&mut buffer).unwrap();

    buffer.seek(0).unwrap();
    let parsed_record = DnsRecord::read(&mut buffer).unwrap();

    assert_eq!(record, parsed_record);
}

#[test]
fn test_dns_record_ns() {
    let mut buffer = BytePacketBuffer::default();
    let record = DnsRecord::NS {
        domain: "example.com".to_string(),
        host: "ns1.example.com".to_string(),
        ttl: 3600,
    };
    record.write(&mut buffer).unwrap();

    buffer.seek(0).unwrap();
    let parsed_record = DnsRecord::read(&mut buffer).unwrap();

    assert_eq!(record, parsed_record);
}

#[test]
fn test_dns_record_cname() {
    let mut buffer = BytePacketBuffer::default();
    let record = DnsRecord::CNAME {
        domain: "www.example.com".to_string(),
        host: "example.com".to_string(),
        ttl: 3600,
    };
    record.write(&mut buffer).unwrap();

    buffer.seek(0).unwrap();
    let parsed_record = DnsRecord::read(&mut buffer).unwrap();

    assert_eq!(record, parsed_record);
}

#[test]
fn test_dns_record_mx() {
    let mut buffer = BytePacketBuffer::default();
    let record = DnsRecord::MX {
        domain: "example.com".to_string(),
        priority: 10,
        host: "mail.example.com".to_string(),
        ttl: 3600,
    };
    record.write(&mut buffer).unwrap();

    buffer.seek(0).unwrap();
    let parsed_record = DnsRecord::read(&mut buffer).unwrap();

    assert_eq!(record, parsed_record);
}

#[test]
fn test_dns_record_unknown() {
    let mut buffer = BytePacketBuffer::default();
    let record = DnsRecord::UNKNOWN {
        domain: "example.com".to_string(),
        qtype: 99,
        data_len: 4,
        ttl: 3600,
    };
    record.write(&mut buffer).unwrap();

    buffer.seek(0).unwrap();
    let parsed_record = DnsRecord::read(&mut buffer).unwrap();

    assert_eq!(record, parsed_record);
}
