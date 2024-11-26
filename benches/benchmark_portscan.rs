use async_std::task::block_on;
use criterion::{criterion_group, criterion_main, Criterion};
use rustscan::input::{PortRange, ScanOrder};
use rustscan::port_strategy::PortStrategy;
use rustscan::scanner::Scanner;
use std::hint::black_box;
use std::net::IpAddr;
use std::time::Duration;

fn get_scanner(udp: bool) -> Scanner {
    let addrs = vec!["127.0.0.1".parse::<IpAddr>().unwrap()];
    let range = PortRange {
        start: 1,
        end: 1023,
    };
    let strategy = PortStrategy::pick(&Some(range), None, ScanOrder::Serial);

    return Scanner::new(
        &addrs,
        10,
        Duration::from_millis(100),
        1,
        false,
        strategy,
        true,
        vec![],
        udp,
    );
}

fn benchmark_tcp_scanner(c: &mut Criterion) {
    let tcp_scanner = get_scanner(false);

    c.bench_function("portscan tcp", |b| {
        b.iter(|| black_box(block_on(tcp_scanner.run())))
    });
}

fn benchmark_udp_scanner(c: &mut Criterion) {
    let mut group = c.benchmark_group("portscan udp");
    group.measurement_time(Duration::from_secs(10));

    let udp_scanner = get_scanner(true);

    group.bench_function("portscan udp", |b| {
        b.iter(|| black_box(block_on(udp_scanner.run())))
    });
}

criterion_group!(benches, benchmark_tcp_scanner, benchmark_udp_scanner);
criterion_main!(benches);
