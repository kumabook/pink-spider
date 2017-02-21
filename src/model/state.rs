use std::fmt;
use rustc_serialize::json::{ToJson, Json};

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub enum State {
    Alive,
    Dead,
}

impl PartialEq for State {
    fn eq(&self, p: &State) -> bool {
        match *self {
            State::Alive => match *p { State::Alive => true, _ => false },
            State::Dead  => match *p { State::Dead  => true, _ => false },
        }
    }
}

impl State {
    fn to_string(&self) -> String {
        match *self {
            State::Alive => "alive",
            State::Dead  => "dead",
        }.to_string()
    }
    pub fn new(str: String) -> State {
        match str.as_ref() {
            "alive" => State::Alive,
            "dead"  => State::Dead,
            _       => State::Dead,
        }
    }
}

impl ToJson for State {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
