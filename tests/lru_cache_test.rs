use std::{thread, time::Duration};

use rdns_resolver_rs::{dns_packet::DnsPacket, dns_record::DnsRecord, lru_cache::LRUCache};

fn sample_dns_packet(ttl: u32) -> DnsPacket {
    let mut packet = DnsPacket::default();
    packet.answers = vec![DnsRecord::A {
        domain: "example.com".to_string(),
        addr: "93.184.216.34".parse().unwrap(),
        ttl,
    }];
    packet
}

#[test]
fn test_put_and_get() {
    let mut cache = LRUCache::new(2);
    let key = "example.com".to_string();
    let packet = sample_dns_packet(10);

    cache.put(&key, &packet);
    let retrieved = cache.get(&key);

    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().answers.len(), 1);
}

#[test]
fn test_lru_eviction() {
    let mut cache = LRUCache::new(2);
    cache.put(&"key1".to_string(), &sample_dns_packet(10));
    cache.put(&"key2".to_string(), &sample_dns_packet(10));
    cache.put(&"key3".to_string(), &sample_dns_packet(10));

    assert!(cache.get(&"key1".to_string()).is_none());
    assert!(cache.get(&"key2".to_string()).is_some());
    assert!(cache.get(&"key3".to_string()).is_some());
}

#[test]
fn test_ttl_expiry() {
    let mut cache = LRUCache::new(2);
    let key = "example.com".to_string();
    cache.put(&key, &sample_dns_packet(1));

    thread::sleep(Duration::from_secs(1));
    let retrieved = cache.get(&key);

    assert!(retrieved.is_none());
}

#[test]
fn test_update_existing_key() {
    let mut cache = LRUCache::new(2);
    let key = "example.com".to_string();
    cache.put(&key, &sample_dns_packet(10));
    cache.put(&key, &sample_dns_packet(20));

    let retrieved = cache.get(&key);
    assert!(retrieved.is_some());
}

#[test]
fn test_remove_key() {
    let mut cache = LRUCache::new(2);
    let key = "example.com".to_string();
    cache.put(&key, &sample_dns_packet(10));
    cache.remove(&key);

    assert!(cache.get(&key).is_none());
}
