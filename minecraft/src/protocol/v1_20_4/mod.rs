use std::io::{Error, ErrorKind, Result};

use bytes::{Buf, BytesMut};
use common_server::{packet::PacketHandler, selector::Socket, var_int::VarIntRead};

use crate::{
    packet_read_handler::PacketReadHandler, player::Player,
    server::Server, session_relay::ConnectionState,
};

use self::{
    handshake::HandShake, login_start::LoginStart, ping::PingRequest, status::StatusRequest,
};

pub mod handshake;
pub mod login_start;
pub mod ping;
pub mod status;

pub struct V1_20_4;

impl PacketReadHandler for V1_20_4 {
    fn handle_packet_read(
        server: &mut Server,
        socket: &mut Socket<Player>,
        value: BytesMut,
    ) -> Result<()> {
        println!("read handle");
        let mut reader = value.reader();
        let packet_id = reader.read_var_i32()?;
        let connection_state = &socket.connection.session_relay.connection_state;
        let bytes = reader.into_inner();
        println!("income: {:?}[{:?}]", connection_state, packet_id);
        match (connection_state, packet_id) {
            (ConnectionState::HandShake, 0) => {
                HandShake::try_from(bytes)?.handle_packet(server, socket)?
            }
            (ConnectionState::Login, 0) => {
                LoginStart::try_from(bytes)?.handle_packet(server, socket)?
            }
            (ConnectionState::Status, 0) => {
                StatusRequest::new().handle_packet(server, socket)?
            }
            (ConnectionState::Status, 1) => {
                PingRequest::try_from(bytes)?.handle_packet(server, socket)?
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("{:?}[{:?}] not exists", connection_state, packet_id),
                ))
            }
        };
        println!("endincome");
        Ok(())
    }
}
