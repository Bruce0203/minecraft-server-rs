use std::io::{Cursor, Error, ErrorKind, Result};

use bytes::{buf::Reader, Buf, Bytes, BytesMut};

use crate::{
    selector::{Server, Socket},
    var_int::VarIntRead,
    var_string::VarStringRead,
};

struct HandShakeServer {}

#[derive(Default)]
struct HandShakePlayer {}
impl Server<HandShakePlayer> for HandShakeServer {
    fn handle_connection_accept(&mut self) -> HandShakePlayer {
        HandShakePlayer::default()
    }

    fn handle_connection_read(&mut self, socket: &mut Socket<HandShakePlayer>, buf: &[u8]) {
        println!("{:?}", buf);
        let mut reader = BytesMut::from(buf).reader();
        let packet_length = reader.read_var_i32();
        let packet_id = reader.read_var_i32();
        let packet = HandShake::try_from(reader.into_inner());
        println!("{:?}", packet);
        //todo stopship: move to another selector 
    }

    fn handle_connection_closed(&mut self, _socket: &mut Socket<HandShakePlayer>) {}
}

#[derive(Debug)]
struct HandShake {
    protocol_version: i32,
    server_address: String,
    server_port: u16,
    next_state: NextState,
}

impl TryFrom<BytesMut> for HandShake {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> Result<Self> {
        let mut reader = value.reader();
        Ok(HandShake {
            protocol_version: reader.read_var_i32()?,
            server_address: reader.read_var_string::<255>()?,
            server_port: reader.get_mut().get_u16(),
            next_state: NextState::try_from(reader.into_inner())?,
        })
    }
}

#[derive(Debug)]
enum NextState {
    Status = 1,
    Login = 2,
}

impl TryFrom<BytesMut> for NextState {
    type Error = Error;

    fn try_from(mut value: BytesMut) -> std::result::Result<Self, Self::Error> {
        Ok(match value.reader().read_var_i32()? {
            1 => NextState::Status,
            2 => NextState::Login,
            n => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("NextState is {}", n),
                ))
            }
        })
    }
}

#[test]
fn test_handshake_server() {
    println!("Server started!");
    let mut server = HandShakeServer {};
    server.start_selection_loop("127.0.0.1:25565".parse().unwrap(), 100);
}
