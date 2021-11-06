use log::info;

use ureq::AgentBuilder;

const ENDPOINT4: &str = "https://api.ipify.org";
const ENDPOINT6: &str = "https://api64.ipify.org";
const ENDPOINT4J: &str = "https://api.ipify.org?format=json";
const ENDPOINT6J: &str = "https://api64.ipify.org?format=json";

struct Config<T> {
    agent: Option<T>,
}

#[derive(Clone, Copy, Debug)]
enum Engine {
    Ureq,
    Reqw,
}

#[derive(Clone, Copy, Debug)]
struct Api<'a> {
    pub t: Engine,
    pub endp: &'a str,
}

impl<'a> Api<'a> {
    fn new(t: Engine) -> Self {
        Api { t, endp: ENDPOINT6 }
    }

    fn set(mut self, op: Op) -> Self {
        self.endp = match op {
            Op::IP4 => ENDPOINT4,
            Op::IP6 => ENDPOINT6,
            Op::IP4J => ENDPOINT4J,
            Op::IP6J => ENDPOINT6J,
        };
        self
    }

    fn call(self) -> String {
        match self.t {
            Engine::Ureq => {
                let c = AgentBuilder::new().user_agent("ipify/1.0.0").build();
                return c.get(self.endp).call().unwrap().into_string().unwrap();
            }
            Engine::Reqw => {
                let c = reqwest::blocking::ClientBuilder::new()
                    .user_agent("ipify/1.0.0")
                    .build()
                    .unwrap();
                return c.get(self.endp).send().unwrap().text().unwrap();
            }
        };
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    IP4,
    IP6,
    IP4J,
    IP6J,
}

fn doit(a: &Api) {
    let ip4 = a.set(Op::IP4).call();
    let ip6 = a.set(Op::IP6).call();

    println!("IP4={:?}", ip4);
    println!("IP6={:?}", ip6);
}

fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(2)
        .init()
        .unwrap();
    info!("Start");

    info!("Using ureq");
    let a = Api::new(Engine::Ureq);
    doit(&a);

    info!("Using reqwest");
    let e = Api::new(Engine::Reqw);
    doit(&e);
}
