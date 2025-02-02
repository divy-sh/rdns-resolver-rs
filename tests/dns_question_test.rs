use rdns_resolver_rs::{
    byte_packet_buffer::BytePacketBuffer, dns_question::DnsQuestion, query_type::QueryType,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write() {
        let mut buffer = BytePacketBuffer::default();
        let question = DnsQuestion::new("example.com".to_string(), QueryType::A);

        let result = question.write(&mut buffer);
        assert!(result.is_ok());

        let expected = vec![
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0, 0, 1, 0, 1,
        ];
        assert_eq!(buffer.pos, expected.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], buffer.buf[i]);
        }
        for i in expected.len()..512 {
            assert_eq!(0, buffer.buf[i]);
        }
    }

    #[test]
    fn test_read() {
        let mut buffer = BytePacketBuffer::default();
        let question = DnsQuestion::new("example.com".to_string(), QueryType::A);

        let _ = question.write(&mut buffer);

        let mut resp = DnsQuestion::new("".to_string(), QueryType::A);
        let result = resp.read(&mut buffer);

        assert!(result.is_ok());
        assert_eq!(question.name, "example.com");
        assert_eq!(question.qtype, QueryType::A);
    }
}
