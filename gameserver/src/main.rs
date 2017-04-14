extern crate rustc_serialize;
extern crate docopt;
extern crate tokio_proto;
extern crate shotgun_common;

//use std::io::prelude::*;
//use std::net::TcpStream;

use std::net::SocketAddr;

use tokio_proto::TcpServer;
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
    let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());

    let touple = format!("[{}]:{}", args.flag_listen, args.flag_port);
    println!("Starting shotgun_gameserver: {}", touple);
    let addr = (&*touple).parse::<SocketAddr>().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(LineProto, addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Echo));
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
