const ENDPOINT4: &str = "https://api.ipify.org";
const ENDPOINT6: &str = "https://api64.ipify.org";
const ENDPOINT4J: &str = "https://api.ipify.org?format=json";
const ENDPOINT6J: &str = "https://api64.ipify.org?format=json";

#[derive(Clone, Copy, Debug)]
pub enum Engine {
    Ureq,
    Reqw,
}

#[derive(Clone, Copy, Debug)]
pub struct Ipify<'a> {
    pub t: Engine,
    pub endp: &'a str,
}

impl<'a> Ipify<'a> {
    pub fn new() -> Self {
        Ipify { t: Engine::Ureq, endp: ENDPOINT6 }
    }

    pub fn with(mut self, e: Engine) -> Self {
        self.t = e;
        self
    }

    pub fn set(mut self, op: Op) -> Self {
        self.endp = match op {
            Op::IP4 => ENDPOINT4,
            Op::IP6 => ENDPOINT6,
            Op::IP4J => ENDPOINT4J,
            Op::IP6J => ENDPOINT6J,
        };
        self
    }

    pub fn call(self) -> String {
        match self.t {
            Engine::Ureq => {
                let c = ureq::AgentBuilder::new().user_agent("ipify/1.0.0").build();
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
pub enum Op {
    IP4,
    IP6,
    IP4J,
    IP6J,
}
