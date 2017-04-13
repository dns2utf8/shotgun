extern crate rustc_serialize;
extern crate docopt;

use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug,RustcDecodable)]
struct Args {
    flag_s     : bool,
    flag_p     : u16,
    arg_IP     : String,
    arg_APIKEY : String,
}

static USAGE: &'static str = "
Usage: shotgun_gameserver [-p PORT] IP APIKEY
Options:
    -p PORT  The port to listen on [default: 3000].
    IP       The socket address to listen on or connect to.
    APIKEY   Your Access Token.
";

fn main() {
    let args: Args = docopt::Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
    
    let port = if args.flag_p == 0 { 3000 } else { args.flag_p };
    
    let mut stream = TcpStream::connect(&*(format!("{}:{}", args.arg_IP, port))).unwrap();
    
    stream.write_all(args.arg_APIKEY.as_bytes()).unwrap();
    stream.write(b"\n");
    stream.flush();
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
