use crate::{byte_packet_buffer::BytePacketBuffer, query_type::QueryType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType,
}

impl DnsQuestion {
    pub fn new(name: String, qtype: QueryType) -> DnsQuestion {
        DnsQuestion { name, qtype }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), String> {
        buffer.read_qname(&mut self.name)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?); // qtype
        let _ = buffer.read_u16()?; // class

        Ok(())
    }

    pub fn write(&self, buffer: &mut BytePacketBuffer) -> Result<(), String> {
        let parts: Vec<&str> = self.name.split('.').collect();
        for part in parts {
            buffer.write(part.len() as u8)?;
            for b in part.bytes() {
                buffer.write(b)?;
            }
        }
        buffer.write(0)?;

        buffer.write_u16(self.qtype.to_num())?;
        buffer.write_u16(1)?; // class

        Ok(())
    }
}
