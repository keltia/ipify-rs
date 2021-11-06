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

const ENDPOINT4: &str = "https://api.ipify.org";
const ENDPOINT6: &str = "https://api64.ipify.org";
const ENDPOINT4J: &str = "https://api.ipify.org?format=json";
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
#[derive(Clone, Copy, Debug)]
pub enum Engine {
    // ureq
    Ureq,
    // reqwest
    Reqw,
}

/// The current set of operations
#[derive(Clone, Copy, Debug)]
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
    pub t: Engine,
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
    ///   let a = Ipify::new();
    ///
    ///   println!("{}", a.call());
    /// }
    /// ```
    ///
    pub fn new() -> Self {
        Ipify {
            t: Engine::Ureq,
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
    ///   let a = Ipify::new().with(Engine::Reqw);
    ///
    ///   println!("{}", a.call());
    /// }
    /// ```
    ///
    pub fn with(mut self, e: Engine) -> Self {
        self.t = e;
        self
    }

    /// Specify the subsequent operation to perform on `call()`
    ///
    /// Examples:
    /// ```
    /// use ipify_rs::{Ipify, Op};
    ///
    /// fn main() {
    ///   let a = Ipify::new().set(Op::IPv6J);
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
        self
    }

    /// Actually perform the API call
    pub fn call(self) -> String {
        match self.t {
            Engine::Ureq => {
                let c = ureq::AgentBuilder::new().user_agent("ipify-cli/1.0.0").build();
                return c.get(self.endp).call().unwrap().into_string().unwrap();
            }
            Engine::Reqw => {
                let c = reqwest::blocking::ClientBuilder::new()
                    .user_agent("ipify-cli/1.0.0")
                    .build()
                    .unwrap();
                return c.get(self.endp).send().unwrap().text().unwrap();
            }
        };
    }
}

