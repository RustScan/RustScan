use async_std::task::block_on;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustscan::input::{PortRange, ScanOrder};
use rustscan::port_strategy::PortStrategy;
use rustscan::scanner::Scanner;
use std::net::IpAddr;
use std::time::Duration;

fn portscan_tcp(scanner: &Scanner) {
    let _scan_result = block_on(scanner.run());
}

fn portscan_udp(scanner: &Scanner) {
    let _scan_result = block_on(scanner.run());
}

fn criterion_benchmark(c: &mut Criterion) {
    let addrs = vec!["127.0.0.1".parse::<IpAddr>().unwrap()];
    let range = PortRange {
        start: 1,
        end: 1_000,
    };
    let strategy_tcp = PortStrategy::pick(&Some(range.clone()), None, ScanOrder::Serial);
    let strategy_udp = PortStrategy::pick(&Some(range.clone()), None, ScanOrder::Serial);

    let scanner_tcp = Scanner::new(
        &addrs,
        10,
        Duration::from_millis(10),
        1,
        false,
        strategy_tcp,
        true,
        vec![],
        false,
    );

    c.bench_function("portscan tcp", |b| {
        b.iter(|| portscan_tcp(black_box(&scanner_tcp)))
    });

    let scanner_udp = Scanner::new(
        &addrs,
        10,
        Duration::from_millis(100),
        1,
        false,
        strategy_udp,
        true,
        vec![],
        true,
    );

    c.bench_function("portscan udp", |b| {
        b.iter(|| portscan_udp(black_box(&scanner_udp)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
