# Shotgun

Connect your bot and play a game.

# Protocol

The protocol is message oriented.
Each message is terminated with a `\n` newline.

## Handshake

![Image of Handshake](handshake.png)

## Multiplexing

After the handshake each line is prefixed with a global game id.

## Grammar

```
ClientHello     := "Programming Language" '\n' "Nickname"
ServerHello     := 'Shotgun Arena Server v0' ProtocolVersion
ProtocolVersion := '0'
Message         := u64 ':' MessageBody
MessageBody     := 'Timeout' | 'Duck' | 'Load' | 'Shoot' | 'Klick' | 'WinRound' | 'LoseRound'
```