extern crate rustc_serialize;
extern crate docopt;
extern crate futures;
extern crate tokio_proto;
extern crate tokio_service;
extern crate shotgun_common;

//use std::io::prelude::*;
//use std::net::TcpStream;

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio_proto::TcpServer;
use shotgun_common::*;
use shotgun_common::ParsedLine::*;
use shotgun_common::networking::*;

#[derive(Debug,RustcDecodable)]
struct Args {
    flag_port  : u16,
    flag_listen: String,
}

static USAGE: &'static str = "
Shotgun Gameserver

Usage:
  shotgun_gameserver [--listen=<IP>] [--port=<PORT>]
  shotgun_gameserver (-h | --help)

Options:
    --port=<PORT>    The port to listen on [default: 6000]
    --listen=<IP>    The socket address to listen on [default: ::1]
";

fn main() {
    // allways print backtrace
    std::env::set_var("RUST_BACKTRACE", "1");

    let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());

    let touple = format!("[{}]:{}", args.flag_listen, args.flag_port);
    println!("Starting shotgun_gameserver: {}", touple);
    let addr = (&*touple).parse::<SocketAddr>().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(LineProto, addr);

    let arena_server = ArenaServer::new();

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(move || Ok(ArenaService {
        server: arena_server.clone(),
    }));
}

pub struct ArenaServer {}

pub struct ArenaService {
    server: Arc<ArenaServer>,
}

impl ArenaServer {
    fn new() -> Arc<ArenaServer> {
        Arc::new(ArenaServer {
        })
    }
}

use tokio_service::Service;
use futures::{future, Future, BoxFuture};

impl Service for ArenaService {
    // These types must match the corresponding protocol types:
    type Request = ParsedLine;
    type Response = ParsedLine;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        println!("call: {:?}", req);

        if let MultiplexedMessage { game_id, action} = req {
            let resp = MultiplexedMessage {
                game_id: game_id,
                action: Action::Load,
            };
            // In this case, the response is immediate.
            future::ok(resp).boxed()
        } else {
            future::err(io::Error::new(io::ErrorKind::Other, "invalid client state")).boxed()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
