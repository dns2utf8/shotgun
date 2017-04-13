extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

pub mod networking;

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


impl From<String> for ParsedLine {
    fn from(input: String) -> ParsedLine {
        ParsedLine {
            game_id: 0,
            action: Timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_zero_duck() {
        let resp = From::from()
    }
}
