# Shotgun

Connect your bot and play a game.

Currently this repo is under heavy development.
It will be transfered to Github once it reaches a stable phase.

# Protocol

The protocol is message oriented.
Each message is terminated with a `\n` newline.

## Handshake

The handshake must be initialized from the client by sending a correct `ClientHello`.
After the server responds with the `ServerHello` the conversation enters the multiplexed phase.

## Multiplexing

After the handshake each line is prefixed with a global game id.

Offcourse there is no rule without exception:
The `RequestNewGame` message does not need to be multiplexed.

## Grammar

```
ClientHello        := 'Nickname: >' "Nickname" '<>' "Programming Language"
ServerHello        := 'Shotgun Arena Server v' ProtocolVersion ' :: max round length[ms]: ' u64
ProtocolVersion    := '0'
RequestNewGame     := 'RequestNewGame'
MultiplexedMessage := u64 ':' Action
Action             := 'NewGame { player_name_a: ' String ', player_name_b: ' String ' }' | 'WinGame' | 'LoseGame' | RoundAction | 'RoundResult { a: ' RoundAction ', b: ' RoundAction ' }' | 'ErrorEnd'
RoundAction        := 'Timeout' | 'Duck' | 'Load' | 'Shoot' | 'Klick'
```

## Example communication

Client messages are prefixed with `< `, server messages with `> `.
All messages are terminated with `\n`:

```
< ClientHello { nickname: "me" }
> ServerHello
< RequestNewGame
> '13:NewGame { player_name_a: "me", player_name_b: "some bot" }'
< '13:Load'
> '13:RoundResult { a: Load, b: Load }'
< '13:Shoot'
> '13:RoundResult { a: Shoot, b: Load }'
> '13:WinGame'
```

# TODO

  - [X] Define protocol
  - [X] Handshake
  - [X] Correct bot
  - [ ] Server match-makeing
  - [ ] Server releay communication / Arena mode
