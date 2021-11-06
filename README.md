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

## crates.io
You can use this package in your project by adding the following
to your `Cargo.toml`:

``` toml
[dependencies]
ipify-rs = "0.2.0"
```
then you can use it in your own crates.
