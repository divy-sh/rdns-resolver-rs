use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::LazyLock;

pub struct RootNameServer {
    pub name: String,
    pub a: Ipv4Addr,
    pub aaaa: Ipv6Addr,
    pub ttl: u32,
}

pub static MAX_RECURSION_DEPTH: i32 = 1;

pub static ROOT_NAME_SERVERS: LazyLock<[RootNameServer; 13]> = LazyLock::new(|| {
    [
        RootNameServer {
            name: "A.ROOT-SERVERS.NET".to_string(),
            a: "198.41.0.4".parse().unwrap(),
            aaaa: "2001:503:ba3e::2:30".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "B.ROOT-SERVERS.NET".to_string(),
            a: "170.247.170.2".parse().unwrap(),
            aaaa: "2801:1b8:10::b".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "C.ROOT-SERVERS.NET".to_string(),
            a: "192.33.4.12".parse().unwrap(),
            aaaa: "2001:500:2::c".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "D.ROOT-SERVERS.NET".to_string(),
            a: "199.7.91.13".parse().unwrap(),
            aaaa: "2001:500:2d::d".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "E.ROOT-SERVERS.NET".to_string(),
            a: "192.203.230.10".parse().unwrap(),
            aaaa: "2001:500:a8::e".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "F.ROOT-SERVERS.NET".to_string(),
            a: "192.5.5.241".parse().unwrap(),
            aaaa: "2001:500:2f::f".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "G.ROOT-SERVERS.NET".to_string(),
            a: "192.112.36.4".parse().unwrap(),
            aaaa: "2001:500:12::d0d".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "H.ROOT-SERVERS.NET".to_string(),
            a: "198.97.190.53".parse().unwrap(),
            aaaa: "2001:500:1::53".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "I.ROOT-SERVERS.NET".to_string(),
            a: "192.36.148.17".parse().unwrap(),
            aaaa: "2001:7fe::53".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "J.ROOT-SERVERS.NET".to_string(),
            a: "192.58.128.30".parse().unwrap(),
            aaaa: "2001:503:c27::2:30".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "K.ROOT-SERVERS.NET".to_string(),
            a: "193.0.14.129".parse().unwrap(),
            aaaa: "2001:7fd::1".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "L.ROOT-SERVERS.NET".to_string(),
            a: "199.7.83.42".parse().unwrap(),
            aaaa: "2001:500:9f::42".parse().unwrap(),
            ttl: 3600000,
        },
        RootNameServer {
            name: "M.ROOT-SERVERS.NET".to_string(),
            a: "202.12.27.33".parse().unwrap(),
            aaaa: "2001:dc3::35".parse().unwrap(),
            ttl: 3600000,
        },
    ]
});
