//use std::io::prelude::*;
//use std::net::TcpStream;
use std::io;
use std::str;
use bytes::{BytesMut, BufMut};
use tokio_io::codec::{Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;

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
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
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
    type Request = String;

    /// For this protocol style, `Response` matches the coded `Out` type
    type Response = String;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}


pub struct Echo;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};

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
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
