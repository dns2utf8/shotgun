/// Connect to the server and play the game with the most defensive strategy


extern crate shotgun_common;

use shotgun_common::*;

#[derive(Debug,RustcDecodable)]
struct Args {
    flag_s     : bool,
    flag_p     : u16,
    arg_IP     : String,
    arg_APIKEY : String,
}

static USAGE: &'static str = "
Usage: shotgun_coward_bot [-p PORT] IP APIKEY
Options:
    -p PORT  The port to listen on [default: 3000].
    IP       The socket address to listen on or connect to.
    APIKEY   Your Access Token.
";

fn main() {

    let server_connection = ServerConnection::connect("localhost", None, "coward_bot").unwrap();

    server_connection.onLineReceived(|action| {
        println!("{:?}", action);

    });
}
