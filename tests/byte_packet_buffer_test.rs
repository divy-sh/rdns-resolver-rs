use rdns_resolver_rs::byte_packet_buffer::BytePacketBuffer;

#[test]
fn test_new() {
    let buffer = BytePacketBuffer::new();
    assert_eq!(buffer.pos, 0);
    assert_eq!(buffer.buf.len(), 512);
}

#[test]
fn test_read_write_u8() {
    let mut buffer = BytePacketBuffer::new();
    buffer.write_u8(42).unwrap();
    buffer.seek(0).unwrap();
    assert_eq!(buffer.read().unwrap(), 42);
}

#[test]
fn test_read_write_u16() {
    let mut buffer = BytePacketBuffer::new();
    buffer.write_u16(0x1234).unwrap();
    buffer.seek(0).unwrap();
    assert_eq!(buffer.read_u16().unwrap(), 0x1234);
}

#[test]
fn test_read_write_u32() {
    let mut buffer = BytePacketBuffer::new();
    buffer.write_u32(0x12345678).unwrap();
    buffer.seek(0).unwrap();
    assert_eq!(buffer.read_u32().unwrap(), 0x12345678);
}

#[test]
fn test_write_qname() {
    let mut buffer = BytePacketBuffer::new();
    buffer.write_qname("example.com").unwrap();
    buffer.seek(0).unwrap();

    let mut result = String::new();
    buffer.read_qname(&mut result).unwrap();
    assert_eq!(result, "example.com");
}

#[test]
fn test_set_u16() {
    let mut buffer = BytePacketBuffer::new();
    buffer.set_u16(10, 0xABCD).unwrap();
    assert_eq!(buffer.get(10).unwrap(), 0xAB);
    assert_eq!(buffer.get(11).unwrap(), 0xCD);
}

#[test]
fn test_get_range() {
    let mut buffer = BytePacketBuffer::new();
    buffer.write_u8(1).unwrap();
    buffer.write_u8(2).unwrap();
    buffer.write_u8(3).unwrap();

    let slice = buffer.get_range(0, 3).unwrap();
    assert_eq!(slice, &[1, 2, 3]);
}

#[test]
fn test_read_qname_with_pointer() {
    let mut buffer = BytePacketBuffer::new();
    buffer.set(0, 0xC0).unwrap();
    buffer.set(1, 0x0C).unwrap();
    buffer.set(12, 3).unwrap();
    buffer.set(13, b'w').unwrap();
    buffer.set(14, b'w').unwrap();
    buffer.set(15, b'w').unwrap();
    buffer.set(16, 0).unwrap();
    
    buffer.seek(0).unwrap();
    let mut result = String::new();
    buffer.read_qname(&mut result).unwrap();
    assert_eq!(result, "www");
}