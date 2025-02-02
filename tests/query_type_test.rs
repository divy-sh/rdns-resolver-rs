use rdns_resolver_rs::query_type::QueryType;

#[test]
fn test_to_num() {
    assert_eq!(QueryType::A.to_num(), 1);
    assert_eq!(QueryType::NS.to_num(), 2);
    assert_eq!(QueryType::CNAME.to_num(), 5);
    assert_eq!(QueryType::MX.to_num(), 15);
    assert_eq!(QueryType::AAAA.to_num(), 28);

    let unknown_query = QueryType::UNKNOWN(99);
    assert_eq!(unknown_query.to_num(), 99);
}

#[test]
fn test_from_num() {
    assert_eq!(QueryType::from_num(1), QueryType::A);
    assert_eq!(QueryType::from_num(2), QueryType::NS);
    assert_eq!(QueryType::from_num(5), QueryType::CNAME);
    assert_eq!(QueryType::from_num(15), QueryType::MX);
    assert_eq!(QueryType::from_num(28), QueryType::AAAA);

    let unknown_query = QueryType::from_num(99);
    match unknown_query {
        QueryType::UNKNOWN(num) => assert_eq!(num, 99),
        _ => panic!("Expected QueryType::UNKNOWN(99)"),
    }
}

#[test]
fn test_equality() {
    assert_eq!(QueryType::A, QueryType::A);
    assert_ne!(QueryType::A, QueryType::NS);

    let unknown_query1 = QueryType::UNKNOWN(99);
    let unknown_query2 = QueryType::UNKNOWN(99);
    let unknown_query3 = QueryType::UNKNOWN(100);

    assert_eq!(unknown_query1, unknown_query2);
    assert_ne!(unknown_query1, unknown_query3);
}

#[test]
fn test_clone() {
    let query = QueryType::MX;
    let cloned_query = query.clone();
    assert_eq!(query, cloned_query);
}

#[test]
fn test_debug() {
    let query = QueryType::AAAA;
    let debug_string = format!("{:?}", query);
    assert_eq!(debug_string, "AAAA");
}

use std::collections::HashSet;

#[test]
fn test_hash() {
    let mut set = HashSet::new();
    set.insert(QueryType::A);
    set.insert(QueryType::NS);
    set.insert(QueryType::A);
    assert_eq!(set.len(), 2);
}
