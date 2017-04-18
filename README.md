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
ClientHello     := 'Nickname: >' "Nickname" '<>' "Programming Language"
ServerHello     := 'Shotgun Arena Server v' ProtocolVersion ' :: max round length[ms]: ' u64
ProtocolVersion := '0'
RequestNewGame  := 'RequestNewGame'
Message         := u64 ':' MessageBody
MessageBody     := 'Timeout' | 'Duck' | 'Load' | 'Shoot' | 'Klick' | 'WinRound' | 'LoseRound'
```

# TODO

  - [X] Define protocol
  - [X] Handshake
  - [ ] Correct bot
  - [ ] Server match-makeing
  - [ ] Server releay communication / Arena mode
