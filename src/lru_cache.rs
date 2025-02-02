use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::dns_packet::DnsPacket;

#[derive(Debug)]
pub struct LRUCache {
    capacity: usize,
    map: HashMap<String, Arc<Mutex<Node>>>,
    order: VecDeque<String>,
}

#[derive(Clone, Debug)]
struct Node {
    value: DnsPacket,
    time: Instant,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &String) -> Option<DnsPacket> {
        if let Some(node) = self.map.get(key) {
            let node = node.lock().unwrap();
            // Check if the cache value is expired
            if node.time < Instant::now() {
                // TTL expired, remove the key from the cache
                drop(node);
                self.remove(key);
                return None;
            }
            // Move the key to the front of the order queue
            self.order.retain(|x| x != key);
            self.order.push_front(key.to_string());
            Some(node.value.clone())
        } else {
            None
        }
    }

    pub fn put(&mut self, key: &String, value: &DnsPacket) {
        let ttl = Duration::new(300, 0); // TTL of 300 seconds
                                         // If the key already exists, we update the value and move it to the front of the queue
        if let Some(node) = self.map.get(key) {
            let mut node = node.lock().unwrap();
            node.value = value.clone();
            node.time = Instant::now() + ttl;
            self.order.retain(|x| x != key);
            self.order.push_front(key.clone());
        } else {
            // If the cache is full, remove the least recently used element
            if self.map.len() == self.capacity {
                if let Some(lru_key) = self.order.pop_back() {
                    self.map.remove(&lru_key);
                }
            }
            // Insert the new key-value pair with TTL
            self.map.insert(
                key.clone(),
                Arc::new(Mutex::new(Node {
                    value: value.clone(),
                    time: Instant::now() + ttl,
                })),
            );
            self.order.push_front(key.clone());
        }
    }

    pub fn remove(&mut self, key: &String) {
        // Remove the key from the map
        if self.map.remove(key).is_some() {
            // Remove the key from the order list
            self.order.retain(|x| x != key);
        }
    }
}
