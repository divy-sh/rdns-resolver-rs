use rdns_resolver_rs::res_code::ResultCode;

#[test]
fn test_from_num() {
    assert_eq!(ResultCode::from_num(0), ResultCode::NOERROR);
    assert_eq!(ResultCode::from_num(1), ResultCode::FORMERR);
    assert_eq!(ResultCode::from_num(2), ResultCode::SERVFAIL);
    assert_eq!(ResultCode::from_num(3), ResultCode::NXDOMAIN);
    assert_eq!(ResultCode::from_num(4), ResultCode::NOTIMP);
    assert_eq!(ResultCode::from_num(5), ResultCode::REFUSED);
    assert_eq!(ResultCode::from_num(6), ResultCode::NOERROR);
    assert_eq!(ResultCode::from_num(255), ResultCode::NOERROR);
}
