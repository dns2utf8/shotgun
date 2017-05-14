extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
//extern crate tokio_service;

pub mod networking;

pub use std::time::Duration;

/// Parse like this:
///
/// ```
/// # use shotgun_common::Action;
/// let action: Action = "WinGame".parse().unwrap();
/// ```
#[derive(Debug,PartialEq)]
pub enum Action {
    /// Starts a new game in this game_id with the opponent
    NewGame { player_name_a: String, player_name_b: String },

    /// Ends round and game
    WinGame,
    /// Ends round and game
    LoseGame,

    PlayerInput(RoundAction),
    /// Ends round
    RoundResult { a: RoundAction, b: RoundAction },

    /// Some error happend and this game is over
    ErrorEnd,
}
use Action::*;

/// All the commands clients can send
///
/// ```
/// # use shotgun_common::RoundAction;
/// let action: RoundAction = "Load".parse().unwrap();
/// ```
#[derive(Debug, PartialEq)]
pub enum RoundAction {
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
}
use RoundAction::*;

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
        programming_language: String,
    },
    /// The server may update the duration over time
    ServerHello {
        max_round_length: Duration,
    },

    RequestNewGame,

    /// Messages about a round
    MultiplexedMessage {
        /// Global Game ID
        game_id: u64,
        /// Recieved Action
        action: Action,
    }
}
use ParsedLine::*;

pub struct PlayerState {
    /// Players primary key
    pub nickname: String,
    /// How much bullets are loaded?
    pub ammo_bag: u64,
    /// Is it still alive?
    pub alive: bool,
}


/// Server perspective
pub struct GameState {
    pub game_id: u64,
    /// When creating a new game, this player is first
    pub left_player : Option<PlayerState>,
    /// As soon as this player joins, the game begins
    pub right_player: Option<PlayerState>,
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
            &ClientHello { ref nickname, ref programming_language } => format!("Nickname: >{}<>{}", nickname, programming_language),
            &ServerHello { ref max_round_length } => format!("Shotgun Arena Server v0 :: max round length[ms]: {}", max_round_length.subsec_nanos() / 1_000_000),
            &RequestNewGame => format!("RequestNewGame"),
            &MultiplexedMessage { ref game_id, ref action } => {
                // Obmit the PlayerInput(...)
                if let &PlayerInput(ref command) = action {
                    format!("{}:{:?}", game_id, command)
                } else {
                    format!("{}:{:?}", game_id, action)
                }
            },
        }
    }
    /// This works with `MultiplexedMessage` only!
    pub fn answer(&self, new_action: Action) -> Self {
        match self {
            &MultiplexedMessage { ref game_id, ref action } => MultiplexedMessage { game_id: *game_id, action: new_action },
            _ => panic!("ParsedLine::answer()"),
        }
    }
}

impl std::str::FromStr for ParsedLine {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "RequestNewGame" {
            return Ok(RequestNewGame)
        }

        if s.starts_with("Nickname: >") {
            let mut parts = s.split('>').skip(1);
            let nick = to_parse_error( parts.next() )?;

            return Ok(ClientHello {
                nickname: nick[..nick.len()-1].into(),
                programming_language: to_parse_error( parts.next() )?.into(),
            })
        }

        let banner = "Shotgun Arena Server v0 :: max round length[ms]: ";
        if s.starts_with(banner) {
            let (_, num) = s.split_at(banner.len());

            return Ok(ServerHello {
                max_round_length: Duration::from_millis( num.parse().map_err(|e| InvalidDuration(e))? ),
            })
        }

        let mut parts = s.splitn(2, ':');

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
            "WinGame"       => Ok(WinGame      ),
            "LoseGame"      => Ok(LoseGame     ),
            "ErrorEnd"       => Ok(ErrorEnd      ),
            text => {
                let prefix = "NewGame { player_name_a: \"";
                let suffix = "\" }";
                if text.starts_with(prefix) && text.ends_with(suffix) {
                    let start_a = prefix.len();
                    let end_a = follow_quoted_str(text, start_a +1) -1;
                    let start_b = end_a + "\", player_name_b: \"".len();
                    let end_b = text.len() - suffix.len();
                    Ok(NewGame {
                        player_name_a: text[start_a..end_a].into(),
                        player_name_b: text[start_b..end_b].into(),
                    })

                } else {
                    let prefix = "RoundResult { a: ";
                    let suffix = " }";
                    if text.starts_with(prefix) && text.ends_with(suffix) {
                        let mut iter = text.split_whitespace().skip(3);
                        let (a, _, b) = (iter.next(), iter.next(), iter.next());
                        let mut a = a.unwrap_or("Timeout,").to_string();
                        a.pop();
                        Ok(RoundResult {
                            a: a.parse()?,
                            b: b.unwrap_or("Timeout").parse()?,
                        })

                    } else {
                        if let Ok(pi) = text.parse() {
                            Ok(PlayerInput(pi))
                        } else {
                            let msg = format!("invalid Action: {:?}", text);
                            Err(InvalidAction(msg))
                        }
                    }
                }
            },
        }
    }
}
impl std::str::FromStr for RoundAction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Timeout"        => Ok(Timeout       ),
            "Duck"           => Ok(Duck          ),
            "Load"           => Ok(Load          ),
            "Shoot"          => Ok(Shoot         ),
            "Klick"          => Ok(Klick         ),
            text => {
                let msg = format!("invalid Action: {:?}", text);
                Err(InvalidAction(msg))
            },
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

/// Offset must be at '" and end will be at '"' again
fn follow_quoted_str(buf: &str, offset: usize) -> usize {
    let mut escape = false;
    let mut i = offset + 1;

    for c in buf.as_bytes().iter().skip(i) {
        i += 1;

        match *c {
            b'\\' => { escape = !escape; },
            b'"' if escape => { escape = !escape; },
            b'"' => return i,
            _ => { () },
        };
    }

    i
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn parse_client_hello() {
        let s = "Nickname: >dns2utf8<>rust";
        let obj = ClientHello {
            nickname: "dns2utf8".into(),
            programming_language: "rust".into(),
        };
        assert_eq!(obj, s.parse().unwrap());
    }

    #[test]
    fn encode_client_hello() {
        let s = "Nickname: >dns2utf8<>rust";
        let obj = ClientHello {
            nickname: "dns2utf8".into(),
            programming_language: "rust".into(),
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
    fn parse_request_new_game() {
        let s = "RequestNewGame";
        let obj = RequestNewGame;
        assert_eq!(obj, s.parse().unwrap());
    }
    #[test]
    fn encode_request_new_game() {
        let s = "RequestNewGame";
        let obj = RequestNewGame;
        assert_eq!(s, obj.serialize());
    }

    #[test]
    fn parse_zero_new_game() {
        let s = "0:NewGame { player_name_a: \"me\", player_name_b: \"you\" }";
        let obj = MultiplexedMessage {
            game_id: 0,
            action: NewGame { player_name_a: "me".into(), player_name_b: "you".into() }
        };
        assert_eq!(obj, s.parse().unwrap());
    }
    #[test]
    fn encode_zero_new_game() {
        let s = "0:NewGame { player_name_a: \"me\", player_name_b: \"you\" }";
        let obj = MultiplexedMessage {
            game_id: 0,
            action: NewGame { player_name_a: "me".into(), player_name_b: "you".into() }
        };
        assert_eq!(s, obj.serialize());
    }

    #[test]
    fn parse_ten_duck() {
        let resp = "10:Duck".parse();
        let obj = MultiplexedMessage {
            game_id: 10,
            action: PlayerInput(Duck),
        };
        assert_eq!(Ok(obj), resp);
    }
    #[test]
    fn encode_ten_duck() {
        let pl = MultiplexedMessage {
            game_id: 10,
            action: PlayerInput(Duck),
        };
        assert_eq!("10:Duck".to_string(), pl.serialize());
    }

    #[test]
    fn parse_new_game() {
        let resp = "NewGame { player_name_a: \"me\", player_name_b: \"you\" }".parse().unwrap();
        let obj = NewGame { player_name_a: "me".into(), player_name_b: "you".into() };
        assert_eq!(obj, resp);
    }
    #[test]
    fn encode_new_game() {
        let resp = "NewGame { player_name_a: \"me\", player_name_b: \"you\" }";
        let obj = format!("{:?}", NewGame { player_name_a: "me".into(), player_name_b: "you".into() });
        assert_eq!(resp, obj);
    }

    #[test]
    fn parse_round_result() {
        let resp = "RoundResult { a: Duck, b: Load }".parse().unwrap();
        let obj = RoundResult { a: Duck, b: Load };
        assert_eq!(obj, resp);
    }
    #[test]
    fn encode_round_result() {
        let resp = "RoundResult { a: Duck, b: Load }";
        let obj = format!("{:?}", RoundResult { a: Duck, b: Load });
        assert_eq!(resp, obj);
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
    fn parse_win_game() {
        assert_eq!(Ok(WinGame), "WinGame".parse())
    }
    #[test]
    fn parse_lose_game() {
        assert_eq!(Ok(LoseGame), "LoseGame".parse())
    }
    #[test]
    fn parse_error_end() {
        assert_eq!(Ok(ErrorEnd), "ErrorEnd".parse())
    }
    #[test]
    fn parse_invalid_action() {
        assert_eq!(Err(InvalidAction("invalid Action: \"blubb\"".to_string())), "blubb".parse::<Action>())
    }

    #[test]
    fn check_follow_quoted_str() {
        let s = "ab: \"blubbeln zu zweit\", def: \"asldfj\"";
        let (start, end) = (4, 23);
        assert_eq!(end, follow_quoted_str(&s, start));
        assert_eq!("\"blubbeln zu zweit\"", &s[start..end]);
    }
}
