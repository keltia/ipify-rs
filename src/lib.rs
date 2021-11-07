//! Ipify
//!
//! My implementation of the ipify-cli.org API to get your own public IP address
//!
//! The fastest way to use it is to use the `myip()` wrapper:
//!
//! Example:
//! ```
//! use ipify_rs::myip;
//!
//! fn main() {
//!   println!("My IP is: {}", myip());
//! }
//! ```
//!
//! The full API is described below.

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
/// fn main() {
///     println!("{}", myip())
/// }
/// ```
///
pub fn myip() -> String {
    Ipify::new().call()
}

/// Describe the available HTTP engines
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Engine {
    // ureq
    Ureq,
    // reqwest
    Reqw,
}

/// The current set of operations
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
#[derive(Clone, Copy, Debug)]
pub struct Ipify<'a> {
    /// HTTP Engine
    pub e: Engine,
    /// Current type of operation
    pub t: Op,
    /// Endpoint, different for every operation
    pub endp: &'a str,
}

/// API Implementation
impl<'a> Ipify<'a> {
    /// Create a new API instance client with the defaults
    ///
    /// Examples:
    /// ```
    /// use ipify_rs::*;
    ///
    /// fn main() {
    ///   let mut a = Ipify::new();
    ///
    ///   println!("{}", a.call());
    /// }
    /// ```
    ///
    pub fn new() -> Self {
        Ipify {
            e: Engine::Ureq,
            t: Op::IPv6,
            endp: ENDPOINT6,
        }
    }

    /// Use the specified HTTP engine
    ///
    /// Examples:
    /// ```
    /// use ipify_rs::{Ipify, Engine};
    ///
    /// fn main() {
    ///   let mut a = Ipify::new();
    ///   a.with(Engine::Reqw);
    ///
    ///   println!("{}", a.call());
    /// }
    /// ```
    ///
    pub fn with(mut self, e: Engine) -> Self {
        self.e = e;
        self
    }

    /// Specify the subsequent operation to perform on `call()`
    ///
    /// Examples:
    /// ```
    /// use ipify_rs::{Ipify, Op};
    ///
    /// fn main() {
    ///   let mut a = Ipify::new();
    ///   a.set(Op::IPv6J);
    ///
    ///   println!("{}", a.call());
    /// }
    /// ```
    ///
    pub fn set(mut self, op: Op) -> Self {
        self.endp = match op {
            Op::IPv4 => ENDPOINT4,
            Op::IPv6 => ENDPOINT6,
            Op::IPv4J => ENDPOINT4J,
            Op::IPv6J => ENDPOINT6J,
        };
        self.t = op;
        self
    }

    /// Actually perform the API call
    pub fn call(self) -> String {
        match self.e {
            Engine::Ureq => {
                let c = ureq::AgentBuilder::new()
                    .user_agent("ipify-cli/1.0.0")
                    .build();
                return c.get(&self.endp).call().unwrap().into_string().unwrap();
            }
            Engine::Reqw => {
                let c = reqwest::blocking::ClientBuilder::new()
                    .user_agent("ipify-cli/1.0.0")
                    .build()
                    .unwrap();
                return c.get(self.endp).send().unwrap().text().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let c = Ipify::new().set(Op::IPv4J).set(Op::IPv6J);
        assert_eq!(Op::IPv6J, c.t);
    }

    #[test]
    fn test_with_1() {
        let c = Ipify::new();

        assert_eq!(Op::IPv6, c.t);
        let c = c.with(Engine::Reqw);
        assert_eq!(Engine::Reqw, c.e);
        let c = c.with(Engine::Ureq);
        assert_eq!(Engine::Ureq, c.e);
    }

    #[test]
    fn test_with_set() {
        let c = Ipify::new();

        assert_eq!(Op::IPv6, c.t);
        let c = c.with(Engine::Reqw).set(Op::IPv4);
        assert_eq!(Engine::Reqw, c.e);
        assert_eq!(Op::IPv4, c.t);

        let c = c.set(Op::IPv4J).with(Engine::Ureq);
        assert_eq!(Engine::Ureq, c.e);
        assert_eq!(Op::IPv4J, c.t);
    }
}
