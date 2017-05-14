'use strict';

const max_games = 1;
var game_states = {};

const LOAD = 'Load'
  , SHOOT = 'Shoot'
  , WIN = 'WinRound'
  , LOOSE = 'LooseRound'
  , NEW_GAME = 'NewGame'
  , REQUEST_NEW_GAME = 'RequestNewGame'
  , CLIENT_HELLO = 'Nickname: >dns2utf8<>javascript'
  , SERVER_HELLO = 'Shotgun Arena Server v0 :: max round length[ms]: 200'


const net = require('net');

const client = net.connect({host: '::1', port: 6000}, () => {
  // 'connect' listener
  console.log('connected to server!');
  client.write(CLIENT_HELLO + '\n');
});

var buf = '';
client.on('data', (data) => {
  buf += data.toString();

  var next_nl = -1;
  while ((next_nl = buf.indexOf('\n')) !== -1) {
    var msg = buf.substr(0, next_nl);
    buf = buf.substr(next_nl + 1);
    proto_handler(msg);
  }
});
client.on('end', () => {
  console.log('disconnected from server');
});
client.on('error', (err) => {
  console.log('ERROR: ' + err);
});

function proto_handler(msg) {
  //console.log('> '+msg);

  if (msg === SERVER_HELLO) {
    for (var i = 0; i < max_games; ++i) {
      client.write('RequestNewGame\n');
    }
  } else {
    const seperator = msg.indexOf(':');
    const game_id = parseInt( msg.substr(0, seperator) );
    const command = msg.substr(seperator + 1);

    console.log('> '+ [game_id, command])
    if (command.startsWith('NewGame')) {
      game_states[game_id] = {};
      respond(game_id, LOAD);
    } else if (command.startsWith(WIN) || command.startsWith(LOOSE)) {
      client.write('RequestNewGame\n');
    } else {
      var game = game_states[game_id];
      if (game.bullets > 0) {
        respond(game_id, SHOOT);
      } else {
        respond(game_id, LOAD);
      }
    }
  }
}

function respond(game_id, command) {
  const answer = ''+game_id + ':' + command;
  client.write(answer);
  console.log('< ' + answer);

  game_states[game_id].last_command = command;
  if (command === LOAD) {
    game_states[game_id].bullets += 1;
  }
  if (command === SHOOT) {
    game_states[game_id].bullets -= 1;
  }
}
