use clap::{crate_name, crate_version, crate_authors, AppSettings, Parser};
use log::info;
use ipify_rs::{Engine, Ipify, Op};

/// Binary name
pub(crate) const NAME: &str = "ipify";
/// Binary version, different from the API itself represented the crate.
pub(crate) const VERSION: &str = "0.1.0";

/// Help message
#[derive(Debug, Parser)]
#[clap(name = NAME, about = "Rust CLI for IPIFY API.")]
#[clap(version = VERSION, author = crate_authors!())]
#[clap(setting = AppSettings::NoAutoVersion)]
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

fn main() {
    let opts: Opts = Opts::parse();

    // Do not forget to set NoAutoVersion otherwise this is ignored
    if opts.version {
        let n = crate_name!();
        let v = crate_version!();

        println!("Running API {}/{} CLI {}/{}\n", n, v, NAME, VERSION);
        std::process::exit(0);
    }

    info!("Start");
}
