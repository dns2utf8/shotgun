extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

pub mod networking;

/// Parse like this:
///
/// ```
/// # use shotgun_common::Action;
/// let action: Action = "Load".parse().unwrap();
/// ```
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

    /// Ends round
    WinRound,
    /// Ends round
    LoseRound,
    /// Ends round
    StalemateRound,

    /// Some error happend and this game is over
    ErrorEnd,
}
use Action::*;

/// Implements parsing and encoding:
///
/// ```
/// # use shotgun_common::ParsedLine;
/// let action: ParsedLine = "42:Load".parse().unwrap();
/// ```
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

#[derive(Debug,PartialEq)]
pub enum ParseError {
    InvalidAction(String),
    InvalidGameId(std::num::ParseIntError),
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
            game_id: parts.next().unwrap().parse().map_err(|e| InvalidGameId(e))?,
            action:  parts.next().unwrap().parse()?,
        })
    }
}

impl std::str::FromStr for Action {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Timeout"        => Ok(Timeout       ),
            "Duck"           => Ok(Duck          ),
            "Load"           => Ok(Load          ),
            "Shoot"          => Ok(Shoot         ),
            "Klick"          => Ok(Klick         ),
            "WinRound"       => Ok(WinRound      ),
            "LoseRound"      => Ok(LoseRound     ),
            "StalemateRound" => Ok(StalemateRound),
            "ErrorEnd"       => Ok(ErrorEnd      ),
            text       => Err(InvalidAction(
                                format!("invalid Action: {:?}", text))),
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

    #[test]
    fn parse_timeout() {
        assert_eq!(Ok(Timeout), "Timeout".parse())
    }
    #[test]
    fn parse_duck() {
        assert_eq!(Ok(Duck), "Duck".parse())
    }
    #[test]
    fn parse_load() {
        assert_eq!(Ok(Load), "Load".parse())
    }
    #[test]
    fn parse_shoot() {
        assert_eq!(Ok(Shoot), "Shoot".parse())
    }
    #[test]
    fn parse_klick() {
        assert_eq!(Ok(Klick), "Klick".parse())
    }
    #[test]
    fn parse_win_round() {
        assert_eq!(Ok(WinRound), "WinRound".parse())
    }
    #[test]
    fn parse_lose_round() {
        assert_eq!(Ok(LoseRound), "LoseRound".parse())
    }
    #[test]
    fn parse_stalemate_round() {
        assert_eq!(Ok(StalemateRound), "StalemateRound".parse())
    }
    #[test]
    fn parse_error_end() {
        assert_eq!(Ok(ErrorEnd), "ErrorEnd".parse())
    }
    #[test]
    fn parse_invalid_action() {
        assert_eq!(Err(InvalidAction("invalid Action: \"blubb\"".to_string())), "blubb".parse::<Action>())
    }
}
