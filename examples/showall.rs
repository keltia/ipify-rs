use log::info;
use ipify_rs::{Engine, Ipify, Op};

fn doit(e: Engine) {
    let a = Ipify::new().with(e);

    let ip4 = a.set(Op::IP4).call();
    let ip6 = a.set(Op::IP6).call();

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

    info!("Using defaults (ureq, ipv6)");
    println!("IP={}", Ipify::new().call());

    info!("Using defaults, get json");
    println!("IP={}", Ipify::new().set(Op::IP6J).call());

    info!("Using ureq");
    doit(Engine::Reqw);

    info!("Using reqwest");
    doit(Engine::Reqw);
}
