use std::net::Ipv4Addr;

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