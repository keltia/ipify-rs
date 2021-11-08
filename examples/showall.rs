use ipify_rs::*;
use log::info;

fn doit() {
    let ip = Ipify::new();

    println!("IP4={:?}", ip.set(Op::IPv4).call());
    println!("IP6={:?}", ip.set(Op::IPv6).call());
}

fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(2)
        .init()
        .unwrap();
    info!("Start");

    info!("Using default, minimal API");
    println!("IP={}", myip());

    info!("Using defaults (ipv6)");
    println!("IP={}", Ipify::new().call());

    info!("Using defaults, get json");
    println!("IP={}", Ipify::new().set(Op::IPv6J).call());

    doit();
}
