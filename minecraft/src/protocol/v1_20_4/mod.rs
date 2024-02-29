use std::io::{Error, ErrorKind, Result};

use crate::{
    connection::{packet_read_handler::PacketReadHandler, session_relay::ConnectionState},
    player::Player,
    server::Server, protocol::v1_20_4::{login_acknowledged::LoginAcknowledged, plugin_message::PluginMessage},
};

use self::{
    handshake::HandShake, login_start::LoginStart, ping::PingRequest, status::StatusRequest,
};
use bytes::{Buf, BytesMut};
use common_server::{packet::PacketHandler, selector::Socket, var_int::VarIntRead};

pub mod login_acknowledged;
pub mod plugin_message;
pub mod login_success;
pub mod handshake;
pub mod login_start;
pub mod ping;
pub mod status;
pub mod set_compression;

pub struct V1_20_4;

impl<'server> PacketReadHandler<'server> for V1_20_4 {
    fn handle_packet_read(
        server: &mut Server,
        player: &mut Player,
        value: &mut BytesMut,
    ) -> Result<()> {
        let mut reader = value.reader();
        let packet_id = reader.read_var_i32()?;
        let connection_state = &player.session_relay.connection_state;
        let bytes = reader.into_inner();
        println!("read");
        match (connection_state, packet_id) {
            (ConnectionState::HandShake, 0) => {
                HandShake::try_from(bytes)?.handle_packet(server, player)?
            }
            (ConnectionState::Login, 0) => {
                LoginStart::try_from(bytes)?.handle_packet(server, player)?
            }
            (ConnectionState::Status, 0) => {
                StatusRequest::try_from(bytes)?.handle_packet(server, player)?;
            }
            (ConnectionState::Status, 1) => {
                PingRequest::try_from(bytes)?.handle_packet(server, player)?
            }
            (ConnectionState::Login, 3) => LoginAcknowledged::try_from(bytes)?.handle_packet(server, player)?,
            (ConnectionState::Confgiuration, 0x01) => PluginMessage::try_from(bytes)?.handle_packet(server, player)?,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("{:?}[{:?}] not exists", connection_state, packet_id),
                ))
            }
        };
        Ok(())
    }
}
