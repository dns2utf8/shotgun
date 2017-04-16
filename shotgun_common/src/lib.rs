extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

pub mod networking;

use std::time::Duration;

/// Parse like this:
///
/// ```
/// # use shotgun_common::Action;
/// let action: Action = "Load".parse().unwrap();
/// ```
#[derive(Debug,PartialEq)]
pub enum Action {
    /// Starts a new game in this game_id with the opponent
    NewGame { opponent: String },

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
pub enum ParsedLine {
    /// All the informations about a player
    ClientHello {
        nickname: String,
        programming_languege: String,
    },
    /// The server may update the duration over time
    ServerHello {
        max_round_length: Duration,
    },

    /// Messages about a round
    MultiplexedMessage {
        /// Global Game ID
        game_id: u64,
        /// Recieved Action
        action: Action,
    }
}
use ParsedLine::*;

pub struct ClientState {
    pub ammo_bag: u64,
    pub alive: bool,
}

#[derive(Debug,PartialEq)]
pub enum ParseError {
    InvalidAction(String),
    InvalidGameId(std::num::ParseIntError),
    ExpectedValue,
    InvalidDuration(std::num::ParseIntError),
}
use ParseError::*;

impl ParsedLine {
    fn serialize(&self) -> String {
        match self {
            &ClientHello { ref nickname, ref programming_languege } => format!("Nickname: >{}<>{}", nickname, programming_languege),
            &ServerHello { ref max_round_length } => format!("Shotgun Arena Server v0 :: max round length[ms]: {}", max_round_length.subsec_nanos() / 1_000_000),
            &MultiplexedMessage { ref game_id, ref action } => format!("{}:{:?}", game_id, action),
        }
    }
}

impl std::str::FromStr for ParsedLine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Nickname: >") {
            let mut parts = s.split('>').skip(1);
            let nick = to_parse_error( parts.next() )?;

            return Ok(ClientHello {
                nickname: nick[..nick.len()-1].into(),
                programming_languege: to_parse_error( parts.next() )?.into(),
            })
        }

        let banner = "Shotgun Arena Server v0 :: max round length[ms]: ";
        if s.starts_with(banner) {
            let (_, num) = s.split_at(banner.len());

            return Ok(ServerHello {
                max_round_length: Duration::from_millis( num.parse().map_err(|e| InvalidDuration(e))? ),
            })
        }

        let mut parts = s.split(':');

        Ok(MultiplexedMessage {
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

pub fn to_parse_error<T>(o: Option<T>) -> Result<T, ParseError> {
    match o {
        Some(v) => Ok(v),
        None => Err(ExpectedValue),
    }
}

pub fn to_io_err<T>(o: Option<T>) -> Result<T, std::io::Error> {
    use std::io;
    match o {
        Some(v) => Ok(v),
        None => Err(io::Error::new(io::ErrorKind::Other, "expected value")),
    }
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn parse_client_hello() {
        let s = "Nickname: >dns2utf8<>rust";
        let obj = ClientHello {
            nickname: "dns2utf8".into(),
            programming_languege: "rust".into(),
        };
        assert_eq!(obj, s.parse().unwrap());
    }

    #[test]
    fn encode_client_hello() {
        let s = "Nickname: >dns2utf8<>rust";
        let obj = ClientHello {
            nickname: "dns2utf8".into(),
            programming_languege: "rust".into(),
        };
        assert_eq!(s, obj.serialize());
    }

    #[test]
    fn parse_server_hello() {
        let s = "Shotgun Arena Server v0 :: max round length[ms]: 200";
        let obj = ServerHello {
            max_round_length: Duration::from_millis(200),
        };
        assert_eq!(obj, s.parse().unwrap());
    }
    #[test]
    fn encode_server_hello() {
        let s = "Shotgun Arena Server v0 :: max round length[ms]: 200";
        let obj = ServerHello {
            max_round_length: Duration::from_millis(200),
        };
        assert_eq!(s, obj.serialize());
    }

    #[test]
    fn parse_ten_duck() {
        let resp = "10:Duck".parse().unwrap();
        assert_eq!(MultiplexedMessage {
            game_id: 10,
            action: Duck,
        }, resp);
    }

    #[test]
    fn encode_ten_duck() {
        let pl = MultiplexedMessage {
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
