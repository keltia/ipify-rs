use clap::{crate_authors, crate_name, crate_version, Parser};
use ipify_rs::{Ipify, Op};

/// Binary name
pub(crate) const NAME: &str = "ipify-cli";
/// Binary version, different from the API itself represented the crate.
pub(crate) const VERSION: &str = "0.4.1";

/// Help message
#[derive(Debug, Parser)]
#[command(disable_version_flag = true)]
#[clap(name = NAME, about = "Rust CLI for IPIFY API.")]
#[clap(version = VERSION, author = crate_authors!())]
struct Opts {
    /// Quiet mode
    #[clap(short = 'q', long)]
    quiet: bool,
    /// Display version and exit
    #[clap(short = 'V', long = "version")]
    version: bool,
    /// Force getting IPv4
    #[clap(short = '4', long = "ipv4")]
    ipv4: bool,
    /// Force getting IPv6
    #[clap(short = '6', long = "ipv6")]
    ipv6: bool,
    /// Request JSON output
    #[clap(short = 'J', long = "json")]
    json: bool,
}

fn banner() -> String {
    let n = crate_name!();
    let v = crate_version!();

    format!("CLI {}/{} using API {}/{}\n", NAME, VERSION, n, v)
}

/// Start
fn main() -> Result<(), ()> {
    let opts: Opts = Opts::parse();

    let v = !opts.quiet;

    // Do not forget to set NoAutoVersion otherwise this is ignored
    if opts.version {
        println!("{}", banner());
        std::process::exit(0);
    }

    if v {
        println!("{}", banner())
    }

    // Start with defaults
    let mut op = Op::IPv6;

    if opts.ipv4 {
        op = Op::ForcedIPv4;
    }

    if opts.ipv6 {
        op = Op::ForcedIPv6;
    }

    if opts.json {
        op = match op {
            Op::IPv4 | Op::IPv4J => Op::IPv4J,
            Op::IPv6 | Op::IPv6J => Op::IPv6J,
            Op::ForcedIPv4 | Op::ForcedIPv4J => Op::ForcedIPv4J,
            Op::ForcedIPv6 | Op::ForcedIPv6J => Op::ForcedIPv6J,
        };
    }

    let c = Ipify::new();
    let r = c.set(op).call();
    if v {
        println!("My IP = {}", r);
    } else {
        println!("{}", r);
    }
    Ok(())
}
