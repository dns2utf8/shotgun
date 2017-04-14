extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

pub mod networking;

#[derive(Debug,PartialEq)]
pub enum Action {
    /// Client did not respont this round
    Timeout,
    /// Hide, you can not be hit
    Duck,
    /// Load one bullet into your magazine, you can be hit
    Load,
    /// Shoot one bullet, may fail when magazine is empty
    Shoot,
    /// Result, when an opponent tried to shoot without a bullet
    Klick,
    WinRound,
    LoseRound,
}
use Action::*;

#[derive(Debug,PartialEq)]
pub struct ParsedLine {
    /// Global Game ID
    pub game_id: u64,
    /// Recieved Action
    pub action: Action,
}

pub struct ClientState {
    pub ammo_bag: u64,
    pub alive: bool,
}

#[derive(Debug)]
pub enum ParseError {
    IlligalAction(String),
    InvalidNumber(std::num::ParseIntError),
}
use ParseError::*;

impl ParsedLine {
    fn serialize(&self) -> String {
        format!("{}:{:?}", self.game_id, self.action)
    }
}

impl std::str::FromStr for ParsedLine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');

        Ok(ParsedLine {
            game_id: parts.next().unwrap().parse().map_err(|e| InvalidNumber(e))?,
            action:  parts.next().unwrap().parse()?,
        })
    }
}

impl std::str::FromStr for Action {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Timeout" => Ok(Timeout),
            "Duck"    => Ok(Duck   ),
            "Load"    => Ok(Load   ),
            text      => Err(IlligalAction(
                                format!("invalid Action: {}", text))),
        }
    }
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn parse_ten_duck() {
        let resp = "10:Duck".parse().unwrap();
        assert_eq!(ParsedLine {
            game_id: 10,
            action: Duck,
        }, resp);
    }

    #[test]
    fn encode_ten_duck() {
        let pl = ParsedLine {
            game_id: 10,
            action: Duck,
        };
        assert_eq!("10:Duck".to_string(), pl.serialize());
    }
}
