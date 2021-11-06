use clap::{crate_authors, crate_name, crate_version, AppSettings, Parser};
use log::info;

/// Binary name
pub(crate) const NAME: &str = "ipify-cli";
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

fn banner() -> String {
    let n = crate_name!();
    let v = crate_version!();

    format!("CLI {}/{} using API {}/{}\n", NAME, VERSION, n, v)
}
/// Start
fn main() {
    let opts: Opts = Opts::parse();

    let v = !opts.quiet;

    // Do not forget to set NoAutoVersion otherwise this is ignored
    if opts.version {
        println!("{}", banner());
        std::process::exit(0);
    }

    if v { banner() }

}
