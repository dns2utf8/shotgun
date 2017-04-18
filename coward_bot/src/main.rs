/// Connect to the server and play the game with the most defensive strategy
extern crate rustc_serialize;
extern crate docopt;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate shotgun_common;
// #[macro_use] extern crate lazy_static;


use std::io;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::sync::Arc;

use futures::Future;
use tokio_core::net::TcpStream;
use tokio_core::reactor::{Core, Handle};
use tokio_proto::TcpClient;
use tokio_proto::pipeline::{ClientProto, ClientService};
use tokio_service::{Service, NewService};

use shotgun_common::*;
use shotgun_common::ParsedLine::*;
use shotgun_common::Action::*;
use shotgun_common::networking::*;

#[derive(Debug,RustcDecodable)]
struct Args {
    flag_port  : u16,
    flag_target: String,
    flag_nickname: String,
}

static USAGE: &'static str = "
Shotgun ClientBot

Usage: 
  shotgun_coward_bot [--target=<IP>] [--port=<PORT>] [--nickname=<NAME>]
  shotgun_coward_bot (-h | --help)

Options:
    --port=<PORT>      The port to listen on [default: 6000]
    --target=<IP>      The socket address to connect to [default: ::1]
    --nickname=<NAME>  The nickname of this instance [default: \"coward_bot\"]
";



fn main() {
    // allways print backtrace
    std::env::set_var("RUST_BACKTRACE", "1");

    let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());
    println!("args: {:?}",args);

    let touple = format!("[{}]:{}", args.flag_target, args.flag_port);

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = (&*touple).to_socket_addrs().unwrap().next().unwrap();

    let client = Client::connect(&addr, &handle)
            .and_then(|client| {
                client.call(ClientHello {
                    nickname: args.flag_nickname.clone(),
                    programming_language: "rust".into(),
                })
                //.for_each(|msg| {})
                .and_then(move |response| {
                    println!(" <-0 {:?}", response);
                    let request = RequestNewGame;
                    println!(" ->0 {:?}", request);
                    client.call(request)
                    .and_then(move |response| {
                        println!(" <-1 {:?}", response);
                        let request = response.answer(Load);
                        println!(" ->1 {:?}", request);
                        client.call(request)
                        .and_then(move |response| {
                            println!(" <-2 {:?}", response);
                            let request = response.answer(Shoot);
                            println!(" ->2 {:?}", request);
                            client.call(request)
                        })
                    })
                })
                /*.and_then(|response| {
                    println!("Server: {:?}", response);
                    client.call(response.answer(Load))
                })*/
            });


    //let (_socket, data) =
    let result = core.run(client);
    println!("\n  Client result: {:?}", result);
    //println!("{}", String::from_utf8_lossy(&data));
}

struct Client {
    inner: ClientService<TcpStream, LineProto>,
}

impl Client {
    /// Establish a connection to a multiplexed line-based server at the
    /// provided `addr`.
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item = Client, Error = io::Error>> {
        let ret = TcpClient::new(LineProto::new())
            .connect(addr, handle)
            .map(move |client_service| {
                //let validate = Validate { inner: client_service};
                //Client { inner: validate }
                Client {
                    inner: client_service,
                }
            });

        Box::new(ret)
    }
}

impl Service for Client {
    type Request = ParsedLine;
    type Response = ParsedLine;
    type Error = io::Error;
    // For simplicity, box the future.
    type Future = Box<Future<Item = ParsedLine, Error = io::Error>>;

    fn call(&self, req: ParsedLine) -> Self::Future {
        Box::new( self.inner.call(req) )
    }
}
