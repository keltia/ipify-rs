use ipify_rs::*;
use log::info;

fn doit(e: Engine) {
    let ip = Ipify::new();
    ip.with(e);

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

    info!("Using defaults (ureq, ipv6)");
    println!("IP={}", Ipify::new().call());

    info!("Using defaults, get json");
    println!("IP={}", Ipify::new().set(Op::IPv6J).call());

    info!("Using ureq");
    doit(Engine::Reqw);

    info!("Using reqwest");
    doit(Engine::Reqw);
}
