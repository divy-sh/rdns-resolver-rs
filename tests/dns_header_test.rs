use rdns_resolver_rs::dns_header::DnsHeader;
use rdns_resolver_rs::byte_packet_buffer::BytePacketBuffer;
use rdns_resolver_rs::res_code::ResultCode;

#[test]
fn test_dns_header_read_write() {
    let mut buffer = BytePacketBuffer::new();
    let header = DnsHeader {
        id: 1234,
        recursion_desired: true,
        truncated_message: false,
        authoritative_answer: true,
        opcode: 2,
        response: true,
        rescode: ResultCode::SERVFAIL,
        checking_disabled: false,
        authed_data: true,
        z: false,
        recursion_available: true,
        questions: 1,
        answers: 2,
        authoritative_entries: 3,
        resource_entries: 4,
    };

    assert!(header.write(&mut buffer).is_ok());
    buffer.pos = 0;

    let mut new_header = DnsHeader::new();
    assert!(new_header.read(&mut buffer).is_ok());

    assert_eq!(header.id, new_header.id);
    assert_eq!(header.recursion_desired, new_header.recursion_desired);
    assert_eq!(header.truncated_message, new_header.truncated_message);
    assert_eq!(header.authoritative_answer, new_header.authoritative_answer);
    assert_eq!(header.opcode, new_header.opcode);
    assert_eq!(header.response, new_header.response);
    assert_eq!(header.rescode, new_header.rescode);
    assert_eq!(header.checking_disabled, new_header.checking_disabled);
    assert_eq!(header.authed_data, new_header.authed_data);
    assert_eq!(header.z, new_header.z);
    assert_eq!(header.recursion_available, new_header.recursion_available);
    assert_eq!(header.questions, new_header.questions);
    assert_eq!(header.answers, new_header.answers);
    assert_eq!(header.authoritative_entries, new_header.authoritative_entries);
    assert_eq!(header.resource_entries, new_header.resource_entries);
}
