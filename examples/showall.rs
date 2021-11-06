use ipify_rs::*;
use log::info;

fn doit(e: Engine) {
    let a = Ipify::new().with(e);

    let ip4 = a.set(Op::IPv4).call();
    let ip6 = a.set(Op::IPv6).call();

    println!("IP4={:?}", ip4);
    println!("IP6={:?}", ip6);
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
