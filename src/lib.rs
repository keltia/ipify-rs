//! Ipify
//!
//! My implementation of the ipify-cli.org API to get your own public IP address
//!
//! The fastest way to use it is to use the `myip()` wrapper:
//!
//! # Example:
//!
//! ```rust
//! use ipify_rs::myip;
//!
//! println!("My IP is: {}", myip());
//! ```
//!
//! The full API is described below.

use clap::{crate_name, crate_version};

/// IPv4 endpoint, plain text
const ENDPOINT4: &str = "https://api.ipify.org";
/// IPv6 endpoint, plain text
const ENDPOINT6: &str = "https://api64.ipify.org";
/// IPv4 endpoint, JSON
const ENDPOINT4J: &str = "https://api.ipify.org?format=json";
/// IPv6 endpoint, JSON
const ENDPOINT6J: &str = "https://api64.ipify.org?format=json";

/// Minimalistic API
///
/// Example:
/// ```
/// use ipify_rs::myip;
///
/// println!("{}", myip())
/// ```
///
#[inline]
pub fn myip() -> String {
    Ipify::new().set(Op::IPv6).call()
}

/// Enumeration for different types of operations provided by the Ipify API.
///
/// Each variant corresponds to a specific operation or request type.
///
/// # Variants
///
/// * `IPv4` - Retrieves the IPv4 address in plain text format.
/// * `IPv6` - Retrieves the IPv6 address in plain text format (default).
/// * `IPv4J` - Retrieves the IPv4 address in JSON format.
/// * `IPv6J` - Retrieves the IPv6 address in JSON format.
///
/// # Examples
///
/// ```
/// use ipify_rs::{Ipify, Op};
///
/// let mut client = Ipify::new();
/// client = client.set(Op::IPv4);
///
/// println!("Public IPv4 address: {}", client.call());
/// ```
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Op {
    /// Plain text
    IPv4,
    /// Plain text (default)
    IPv6,
    /// Json output
    IPv4J,
    /// Json output
    IPv6J,
}

/// The main API struct
///
/// This struct represents a client for interacting with the Ipify API.
/// It allows users to configure and perform operations for retrieving
/// their public IP addresses, either in plain text or JSON format.
///
/// # Fields
///
/// * `t` - The current operation to perform (e.g., IPv4, IPv6, JSON outputs).
/// * `endp` - The API endpoint used for the operation.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use ipify_rs::{Ipify, Op};
///
/// let mut client = Ipify::new();
/// client = client.set(Op::IPv4);
///
/// let ip = client.call();
/// println!("Public IPv4 address: {}", ip);
/// ```
///
/// Using the default settings (IPv6):
///
/// ```rust
/// use ipify_rs::Ipify;
///
/// let ip = Ipify::new().call();
/// println!("Public IPv6 address: {}", ip);
/// ```
///
#[derive(Clone, Debug)]
pub struct Ipify {
    /// Current type of operation
    pub t: Op,
    /// Endpoint, different for every operation
    pub endp: String,
}

/// Impl. default values.
impl Default for Ipify {
    fn default() -> Self {
        Self::new()
    }
}

/// API Implementation
impl Ipify {
    /// Create a new API instance client with the defaults
    ///
    /// # Example:
    /// ```
    /// use ipify_rs::*;
    ///
    /// let a = Ipify::new();
    ///
    /// println!("{}", a.call());
    /// ```
    ///
    pub fn new() -> Self {
        Self {
            t: Op::IPv6,
            endp: ENDPOINT6.to_owned(),
        }
    }

    /// Specify the subsequent operation to perform on `call()`
    ///
    /// # Example:
    /// ```rust
    /// use ipify_rs::{Ipify, Op};
    ///
    /// let mut a = Ipify::new();
    /// a.set(Op::IPv6J);
    ///
    /// println!("{}", a.call());
    /// ```
    ///
    pub fn set(&self, op: Op) -> Self {
        Self {
            t: op,
            endp: match op {
                Op::IPv4 => ENDPOINT4.to_owned(),
                Op::IPv6 => ENDPOINT6.to_owned(),
                Op::IPv4J => ENDPOINT4J.to_owned(),
                Op::IPv6J => ENDPOINT6J.to_owned(),
            },
        }
    }

    /// Actually perform the API call
    ///
    /// # Example:
    /// ```rust
    /// use ipify_rs::Ipify;
    ///
    /// let r = Ipify::new().call();
    ///
    /// println!("my ip = {}", r);
    /// ```
    ///
    pub fn call(self) -> String {
        let c = reqwest::blocking::ClientBuilder::new()
            .user_agent(format!("{}/{}", crate_name!(), crate_version!()))
            .build()
            .unwrap();
        c.get(self.endp).send().unwrap().text().unwrap()
    }

    ///
    /// Perform the API call asynchronously to retrieve the IP address.
    ///
    /// This function communicates with the configured `Ipify` endpoint, sending an
    /// HTTP GET request and retrieving the response body as a string. The result of
    /// the call is typically a public IP address of the client in either plain text
    /// or JSON format, based on the selected operation (`Op`).
    ///
    /// # Example
    ///
    /// ```rust
    /// use ipify_rs::Ipify;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    ///     let ip = Ipify::new().call_async().await;
    ///     println!("My public IP address: {}", ip);
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will panic if:
    /// - The HTTP client fails to build properly (e.g., invalid user-agent).
    /// - The GET request to the endpoint fails (e.g., network error or invalid endpoint).
    /// - The response cannot be transformed into a plain string (e.g., invalid encoding).
    ///
    /// To avoid panics, consider handling errors explicitly by using a custom implementation that propagates errors instead of unwrapping results.
    ///
    pub async fn call_async(self) -> String {
        let c = reqwest::ClientBuilder::new()
            .user_agent(format!("{}/{}", crate_name!(), crate_version!()))
            .build()
            .unwrap();
        c.get(self.endp).send().await.unwrap().text().await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::prelude::*;
    use std::net::IpAddr;

    #[test]
    fn test_set_1() {
        let c = Ipify::new();

        assert_eq!(Op::IPv6, c.t);
        let c = c.set(Op::IPv4J);
        assert_eq!(Op::IPv4J, c.t);
        let c = c.set(Op::IPv6);
        assert_eq!(Op::IPv6, c.t);
    }

    #[test]
    fn test_set_2() {
        let c = Ipify::new();

        let c = c.set(Op::IPv4J).set(Op::IPv6J);
        assert_eq!(Op::IPv6J, c.t);
    }

    #[test]
    fn test_with_1() {
        let c = Ipify::new();

        assert_eq!(Op::IPv6, c.t);
    }

    #[test]
    fn test_with_set() {
        let c = Ipify::new();

        assert_eq!(Op::IPv6, c.t);
        let c = c.set(Op::IPv4);
        assert_eq!(Op::IPv4, c.t);

        let c = c.set(Op::IPv4J);
        assert_eq!(Op::IPv4J, c.t);
    }

    #[test]
    fn test_myip() {
        let server = MockServer::start();

        let m = server.mock(|when, then| {
            when.method(GET).header(
                "user-agent",
                format!("{}/{}", crate_name!(), crate_version!()),
            );
            then.status(200).body("192.0.2.1");
        });

        let mut c = Ipify::new();
        let b = server.base_url().clone();
        c.endp = b.to_owned();
        let str = c.call();

        let ip = str.parse::<IpAddr>();
        m.assert();
        assert!(ip.is_ok());
        assert_eq!("192.0.2.1", str);
    }

    #[tokio::test]
    async fn test_async_call() {
        let server = MockServer::start_async().await;

        let m = server
            .mock_async(|when, then| {
                when.method(GET).header(
                    "user-agent",
                    format!("{}/{}", crate_name!(), crate_version!()),
                );
                then.status(200).body("192.0.2.1");
            })
            .await;

        let mut c = Ipify::new();
        let b = server.base_url().clone();
        c.endp = b.to_owned();
        let str = c.call_async().await;

        let ip = str.parse::<IpAddr>();
        m.assert_async().await;
        assert!(ip.is_ok());
        assert_eq!("192.0.2.1", str);
    }
}
