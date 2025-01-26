<!-- omit in TOC -->

# ipify-rs

> **Rust API & CLI for accessing the ipify.org HTTP API**

[![Cirrus-CI](https://api.cirrus-ci.com/github/keltia/ipify-rs.svg?branch=main)](https://cirrus-ci.org/keltia/ipify-rs)
[![Crates.io](https://img.shields.io/crates/v/ipify-rs.svg)](https://crates.io/crates/ipify-rs)
[![Docs](https://docs.rs/ipify-rs/badge.svg)](https://docs.rs/ipify-rs)
[![GitHub release](https://img.shields.io/github/release/keltia/ipify-rs.svg)](https://github.com/keltia/ipify-rs/releases/)
[![GitHub issues](https://img.shields.io/github/issues/keltia/ipify-rs.svg)](https://github.com/keltia/ipify-rs/issues)
[![SemVer](https://img.shields.io/badge/semver-2.0.0-blue)](https://semver.org/spec/v2.0.0.html)
[![License](https://img.shields.io/crates/l/mit)](https://opensource.org/licenses/MIT)
[![dependency status](https://deps.rs/repo/github/keltia/ipify-rs/status.svg)](https://deps.rs/repo/github/keltia/ipify-rs)

Licensed under the [MIT](LICENSE).

1. [About](#about)
2. [API Usage](#api-usage)
3. [Installation](#installation)
4. [Example](#example)
5. [Documentation](#documentation)
6. [Contributing](#contributing)

## About

This is my attempt at writing an API & CLI client for the IPIFY API (aka ipify.org). After looking at all the other
crates, they are all flawed in some way (only IPv4, not really cargo compliant, etc.).

**Supported Platforms**

* Unix (tested on FreeBSD, Linux and macOS)
* Windows
    * cmd.exe
    * Powershell

## API Usage

**BREAKING CHANGE**: In v0.7 and later, `call()` and `call_async()` returns a `Result<String>` now.

You first create an instance of `Ipify` with `new()` set the result you want (IPv4, IPv6) and its format (plain text,
json). Result is a string inside a `Result`.

```rust
fn main() {
    use ipify_rs::{Ipify, Op};

    let ip = Ipify::new().set(Op::IPv4).call().unwrap();

    println!("My IP is {}", ip);
}
```

The four operations are specified as below:

- `OP::IPv4`
- `OP::IPv6`   (the default)
- `OP::IPv4J`  (json output)
- `Op::IPv6J`  (json output)

### Minimalistic API

If you only care about the default (plain text, IPv6 query) and don't want to reuse anything later, then `myip()` is
what you want:

```rust
use ipify_rs::myip;

fn main() {
    println!("{}", myip());
}
```

### CLI utility

There is a CLI utility bundled with the API called `ipify-cli`.

```text
    ipify-cli 0.5.0
    
    Ollivier Robert <roberto@keltia.net>
    
    Rust CLI for IPIFY API.
    
    USAGE:
        ipify-cli.exe [OPTIONS]
    
    OPTIONS:
        -4, --ipv4       Force getting IPv4
        -6, --ipv6       Force getting IPv6
        -h, --help       Print help information
        -J, --json       Request JSON output
        -q, --quiet      Quiet mode
        -V, --version    Display version and exit
```

You can see both API & CLI versions:

```
    $ ipify-cli -V
CLI ipify-cli/0.5.0 using API ipify-rs/0.7.0

```

## Examples

The file `showall.rs` inside `examples` show almost all parameters for the API. You can run it with:

```text
    $ cargo run --example showall
    ...   
    INFO - Start
    INFO - Using default, minimal API
    IP=aaaa:bbbb:cccc:dddd:eeee:ffff:gggg:hhhh
    INFO - Using defaults (ipv6)
    IP=aaaa:bbbb:cccc:dddd:eeee:ffff:gggg:hhhh
    INFO - Using defaults, get json
    IP={"ip":"aaaa:bbbb:cccc:dddd:eeee:ffff:gggg:hhhh"}
    IP4="A.B.C.D"
    IP6="aaaa:bbbb:cccc:dddd:eeee:ffff:gggg:hhhh"
```

There is an "async" example as well.

## crates.io

You can use this package in your project by adding the following
to your `Cargo.toml`:

``` toml
[dependencies]
ipify-rs = "0.7.0"
```

then you can use it in your own crates.

## Documentation

Full description of the API with examples is on [docs.rs] as usual: [Ipify].

[docs.rs]: https://docs.rs/

[Ipify]: https://docs.rs/ipify-rs

## Contributing

Please see [CONTRIBUTING.md](CONTRIBUTING.md) for some simple rules.

I am now using [Jujutsu] as my VCS. You can do the same or use Git Flow as before.

I use Git Flow for this package so please use something similar or the usual github workflow.

1. Fork it ( https://github.com/keltia/ipify-rs/fork )
2. Checkout the develop branch (`git checkout develop`)
3. Create your feature branch (`git checkout -b my-new-feature`)
4. Commit your changes (`git commit -am 'Add some feature'`)
5. Push to the branch (`git push origin my-new-feature`)
6. Create a new Pull Request

[Jujutsu]: https://jj-vcs.github.io/
