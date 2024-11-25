use async_std::task::block_on;
use criterion::{criterion_group, criterion_main, Criterion};
use rustscan::input::{PortRange, ScanOrder};
use rustscan::port_strategy::PortStrategy;
use rustscan::scanner::Scanner;
use std::net::IpAddr;
use std::time::Duration;

fn portscan_tcp() {
    let addrs = vec!["127.0.0.1".parse::<IpAddr>().unwrap()];
    let range = PortRange {
        start: 1,
        end: 1023,
    };
    let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Serial);
    let scanner = Scanner::new(
        &addrs,
        10,
        Duration::from_millis(100),
        1,
        false,
        strategy,
        true,
        vec![],
        false,
    );

    let _ = block_on(scanner.run());
}

fn portscan_udp() {
    let addrs = vec!["127.0.0.1".parse::<IpAddr>().unwrap()];
    let range = PortRange {
        start: 1,
        end: 1023,
    };
    let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Serial);
    let scanner = Scanner::new(
        &addrs,
        10,
        Duration::from_millis(100),
        1,
        false,
        strategy,
        true,
        vec![],
        true,
    );

    let _ = block_on(scanner.run());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("portscan tcp", |b| b.iter(|| portscan_tcp()));
    c.bench_function("portscan udp", |b| b.iter(|| portscan_udp()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
