use std::net::{Ipv4Addr, Ipv6Addr};

use crate::byte_packet_buffer::BytePacketBuffer;
use crate::query_type::QueryType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum DnsRecord {
    UNKNOWN {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    }, // 0
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    }, // 1
    NS {
        domain: String,
        host: String,
        ttl: u32,
    },
    CNAME {
        domain: String,
        host: String,
        ttl: u32,
    },
    MX {
        domain: String,
        priority: u16,
        host: String,
        ttl: u32,
    },
    AAAA {
        domain: String,
        addr: Ipv6Addr,
        ttl: u32,
    },
}

impl DnsRecord {
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord, String> {
        let domain = String::new();
        buffer.read_qname( domain.clone())?;

        let qtype_num = buffer.read_u16()?;
        let qtype = QueryType::from_num(qtype_num);
        let _ = buffer.read_u16()?;
        let ttl = buffer.read_u32()?;
        let data_len = buffer.read_u16()?;

        match qtype {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr >> 0) & 0xFF) as u8,
                );

                Ok(DnsRecord::A {
                    domain: domain,
                    addr: addr,
                    ttl: ttl,
                })
            }
            QueryType::AAAA => {
                let a = buffer.read_u16()?;
                let b = buffer.read_u16()?;
                let c = buffer.read_u16()?;
                let d = buffer.read_u16()?;
                let e = buffer.read_u16()?;
                let f = buffer.read_u16()?;
                let g = buffer.read_u16()?;
                let h = buffer.read_u16()?;
                let addr = Ipv6Addr::new(a, b, c, d, e, f, g, h);

                Ok(DnsRecord::AAAA {
                    domain: domain,
                    addr: addr,
                    ttl: ttl,
                })
            }
            QueryType::NS => {
                let host = String::new();
                buffer.read_qname(host.clone())?;

                Ok(DnsRecord::NS {
                    domain: domain,
                    host: host,
                    ttl: ttl,
                })
            }
            QueryType::CNAME => {
                let host = String::new();
                buffer.read_qname(host.clone())?;

                Ok(DnsRecord::CNAME {
                    domain: domain,
                    host: host,
                    ttl: ttl,
                })
            }
            QueryType::MX => {
                let priority = buffer.read_u16()?;
                let host = String::new();
                buffer.read_qname(host.clone())?;

                Ok(DnsRecord::MX {
                    domain: domain,
                    priority: priority,
                    host: host,
                    ttl: ttl,
                })
            }
            QueryType::UNKNOWN(_) => {
                buffer.step(data_len as usize)?;

                Ok(DnsRecord::UNKNOWN {
                    domain: domain,
                    qtype: qtype_num,
                    data_len: data_len,
                    ttl: ttl,
                })
            }
        }
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<(), String> {
        match self {
            DnsRecord::A { domain, addr, ttl } => {
                buffer.write_qname(&domain)?;
                buffer.write_u16(QueryType::A.to_num())?;
                buffer.write_u16(1)?; // class
                buffer.write_u32(*ttl)?;
                buffer.write_u16(4)?; // data length
                buffer.write_u8(addr.octets()[0])?;
                buffer.write_u8(addr.octets()[1])?;
                buffer.write_u8(addr.octets()[2])?;
                buffer.write_u8(addr.octets()[3])?;
            }
            DnsRecord::AAAA { domain, addr, ttl } => {
                buffer.write_qname(&domain)?;
                buffer.write_u16(QueryType::AAAA.to_num())?;
                buffer.write_u16(1)?; // class
                buffer.write_u32(*ttl)?;
                buffer.write_u16(16)?; // data length
                for octet in &addr.segments() {
                    buffer.write_u16(*octet)?;
                }
            }
            DnsRecord::NS { domain, host, ttl } => {
                buffer.write_qname(&domain)?;
                buffer.write_u16(QueryType::NS.to_num())?;
                buffer.write_u16(1)?; // class
                buffer.write_u32(*ttl)?;

                buffer.write_u16(host.len() as u16)?;
                buffer.write_qname(&host)?;
            }
            DnsRecord::CNAME { domain, host, ttl } => {
                buffer.write_qname(&domain)?;
                buffer.write_u16(QueryType::CNAME.to_num())?;
                buffer.write_u16(1)?; // class
                buffer.write_u32(*ttl)?;
                buffer.write_u16(host.len() as u16)?;
                buffer.write_qname(&host)?;
            }
            DnsRecord::MX {
                domain,
                priority,
                host,
                ttl,
            } => {
                buffer.write_qname(&domain)?;
                buffer.write_u16(QueryType::MX.to_num())?;
                buffer.write_u16(1)?; // class
                buffer.write_u32(*ttl)?;
                buffer.write_u16(2 + host.len() as u16)?;
                buffer.write_u16(*priority)?;
                buffer.write_qname(&host)?;
            }
            DnsRecord::UNKNOWN {
                domain,
                qtype,
                data_len,
                ttl,
            } => {
                buffer.write_qname(&domain)?;
                buffer.write_u16(*qtype)?;
                buffer.write_u16(1)?; // class
                buffer.write_u32(*ttl)?;
                buffer.write_u16(*data_len)?;
            }
            
        }

        Ok(())
    }
}