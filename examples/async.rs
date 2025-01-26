use ipify_rs::*;
use log::info;

async fn doit() {
    let ip = Ipify::new();

    println!("IP4={:?}", ip.set(Op::IPv4).call_async().await);
    println!("IP6={:?}", ip.set(Op::IPv6).call_async().await);
}

#[tokio::main]
async fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(2)
        .init()
        .unwrap();
    info!("Start");

    info!("Using defaults (ipv6)");
    println!("IP={}", Ipify::new().call_async().await);

    info!("Using defaults, get json");
    println!("IP={}", Ipify::new().set(Op::IPv6J).call_async().await);

    doit().await;
}
