# ipify-rs

[![CircleCI](https://circleci.com/gh/keltia/ipify-rs/tree/main.svg?style=shield)](https://circleci.com/gh/keltia/ipify-rs/tree/main)
[![dependency status](https://deps.rs/repo/github/keltia/ipify-rs/status.svg)](https://deps.rs/repo/github/keltia/ipify-rs)
[![](https://img.shields.io/crates/v/ipify-rs.svg)](https://crates.io/crates/ipify-rs)
[![Docs](https://docs.rs/ipify-rs/badge.svg)](https://docs.rs/ipify-rs)

This is my attempt at writing an API & CLI client for the IPIFY API (aka ipify.org).  After looking at all the other crates, they are all flawed in some way (only IPv4, not really cargo compliant, etc.).

**Supported Platforms**
* Unix (tested on FreeBSD, Linux and macOS)
* Windows
    * cmd.exe
    * Powershell

## API Usage

You first create an instance of `Ipify``with `new()` set the result you want (IPv4, IPv6) and its format (plain text, json).  Result is a string.

```rs
  use ipify::Ipify;
  
  let c = Ipify::new().set(Op::IPv4);
  
  println!("My IP is {}", c.call());
```

## HTTP engine

This API can use either [ureq] or [reqwest] as HTTP client.  You can select the engine with the `with()` method.  The current version of `Ipify` only support the *blocking* client though.

```rs
  use ipify::Ipify;
  
  let c = Ipify::new().with(Engine::Reqw).set(Op::IPv4);
  
  println!("My IP is {}", c.call());
```

[ureq]: https://docs.rs/crate/ureq/
[reqwest]: https://docs.rs/crate/reqwest/

## CLI utility

There is a CLI utility bundled with the API called `ipify`. 
```
    ipify 0.1.0
    
    Ollivier Robert <roberto@keltia.net>
    
    Rust CLI for IPIFY API.
    
    USAGE:
        ipify.exe [OPTIONS]
    
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
    $ ipify -V
    Running API ipify-rs/0.2.0 CLI ipify/0.1.0
```

## Example

The file `showall.rs` inside `examples` show almost all parameters for the API. You can run it with:
```
    $ cargo run --example showall
    ...   
    INFO - Start
    INFO - Using defaults (ureq, ipv6)
    IP=aaaa:bbbb:cccc:dddd:eeee:ffff:gggg:hhhh
    INFO - Using defaults, get json
    IP={"ip":"aaaa:bbbb:cccc:dddd:eeee:ffff:gggg:hhhh"}
    INFO - Using ureq
    IP4="A.B.C.D"
    IP6="aaaa:bbbb:cccc:dddd:eeee:ffff:gggg:hhhh"
    INFO - Using reqwest
    IP4="A.B.C.D"
    IP6="aaaa:bbbb:cccc:dddd:eeee:ffff:gggg:hhhh"
```

## crates.io

You can use this package in your project by adding the following
to your `Cargo.toml`:

``` toml
[dependencies]
ipify-rs = "0.2.0"
```
then you can use it in your own crates.

## Documentation

Full description of the API with examples is on [docs.rs] as usual: [Ipify].

[docs.rs]: https://docs.rs/
[Ipify]: https://docs.rs/ipify-rs
