use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustscan::input::{PortRange, ScanOrder};
use rustscan::port_strategy::PortStrategy;
use rustscan::scanner::Scanner;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

fn bench_port_strategy() {
    let range = PortRange {
        start: 1,
        end: 1_000,
    };
    let _strategy = PortStrategy::range(black_box(range), ScanOrder::Serial);
}

fn criterion_benchmark(c: &mut Criterion) {
    let addrs = [IpAddr::V4(Ipv4Addr::LOCALHOST)];
    let range = PortRange {
        start: 1,
        end: 1_000,
    };
    let strategy_tcp = PortStrategy::range(range, ScanOrder::Serial);
    let strategy_udp = PortStrategy::range(range, ScanOrder::Serial);

    let scanner_tcp = Scanner::new(
        &addrs,
        10,
        Duration::from_millis(10),
        1,
        false,
        strategy_tcp,
        true,
        &[],
        false,
    );

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let mut tcp_group = c.benchmark_group("portscan tcp");
    tcp_group.measurement_time(Duration::from_secs(20));
    tcp_group.sample_size(10);
    tcp_group.bench_function("portscan tcp", |b| {
        b.to_async(&runtime).iter(|| black_box(&scanner_tcp).run())
    });
    tcp_group.finish();

    let scanner_udp = Scanner::new(
        &addrs,
        10,
        Duration::from_millis(10),
        1,
        false,
        strategy_udp,
        true,
        &[],
        true,
    );

    let mut udp_group = c.benchmark_group("portscan udp");
    udp_group.measurement_time(Duration::from_secs(20));
    udp_group.sample_size(25);
    udp_group.bench_function("portscan udp", |b| {
        b.to_async(&runtime).iter(|| black_box(&scanner_udp).run())
    });
    udp_group.finish();
    
    c.bench_function("port strategy", |b| b.iter(bench_port_strategy));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
