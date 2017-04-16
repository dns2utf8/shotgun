//use std::io::prelude::*;
//use std::net::TcpStream;
use std::io;
use std::str;
use bytes::{BytesMut, BufMut};
use futures::{future, Future, BoxFuture, Stream, Sink};
use tokio_io::codec::{Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;

use ::*;

pub enum ConnectionState {
    Connecting,
    Handshake,
    ArenaMode,
}
use self::ConnectionState::*;

/*
pub struct LineFeeder {
    connection: TcpStream,
    nickname: String,
    state: ConnectionState,
}

impl LineFeeder {
    pub fn connect<S: Into<String>>(remote_host_port: (S, u16), nickname: S) -> ::std::io::Result<LineFeeder> {
        let (host, port) = remote_host_port;
        let nickname = nickname.into();

        let mut stream = TcpStream::connect(&*(format!("{}:{}", host.into(), port)))?;

        let client_hello = format!("{}\n{}\n", "rust", nickname);
        stream.write_all(client_hello.as_bytes())?;

        Ok(LineFeeder {
            connection: stream,
            nickname: nickname,
            state: Handshake,
        })
    }
}
*/
pub struct LineCodec;

impl Encoder for LineCodec {
    type Item = ParsedLine;
    type Error = io::Error;

    fn encode(&mut self, msg: ParsedLine, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.serialize().as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

impl Decoder for LineCodec {
    type Item = ParsedLine;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Self::Item>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            let s = str::from_utf8(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, "invalid UTF-8") )
                ?.to_string();

            let line = s.parse()
                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)) )?;

            Ok(Some(line))
        } else {
            Ok(None)
        }
    }
}


pub struct LineProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    /// For this protocol style, `Request` matches the codec `In` type
    type Request = ParsedLine;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = ParsedLine;

    // `Framed<T, LineCodec>` is the return value of
    // `io.framed(LineCodec)`
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Box<Future<Item = Self::Transport,
                                   Error = io::Error>>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let transport = io.framed(LineCodec);

        println!("bind_transport");
//         transport.send("Server Hello".into());

        let handshake = transport.into_future()
            // If the transport errors out, we don't care about
            // the transport anymore, so just keep the error
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                // A line has been received, check to see if it
                // is the handshake
                match line {
                    Some(ref clientHello)/* { nickname, programming_language })*/ => {
                        println!("SERVER: received client handshake");
                        // Send back the acknowledgement
                        let ret = transport.send(ServerHello {
                            max_round_length: Duration::from_millis(200),
                        });
                        Box::new(ret) as Self::BindTransport
                    }
                    _ => {
                        // The client sent an unexpected handshake,
                        // error out the connection
                        println!("SERVER: client handshake INVALID");
                        let err = io::Error::new(io::ErrorKind::Other,
                                                 "invalid handshake");
                        let ret = future::err(err);
                        Box::new(ret) as Self::BindTransport
                    }
                }
            });

        Box::new(handshake)
    }
}


/*
pub struct Echo;
use tokio_service::Service;

impl Service for Echo {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immediate.
        future::ok(req).boxed()
    }
}*/




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
