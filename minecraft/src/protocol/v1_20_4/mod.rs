pub mod client_information;
pub mod feature_flags;
pub mod finish_configuration;
pub mod handshake;
pub mod login_acknowledged;
pub mod login_play;
pub mod login_start;
pub mod login_success;
pub mod ping;
pub mod plugin_message;
pub mod registry;
pub mod set_compression;
pub mod status;

use self::{
    client_information::ClientInformation, finish_configuration::FinishConfiguration,
    handshake::HandShake, login_start::LoginStart, ping::PingRequest, status::StatusRequest,
};
use crate::{
    connection::{player::Player, session_relay::ConnectionState, PacketReadHandler},
    protocol::v1_20_4::{login_acknowledged::LoginAcknowledged, plugin_message::PluginMessage},
    server::Server,
};
use common_server::{packet::PacketHandler, var_int::VarIntRead};
use std::io::{Error, ErrorKind, Result, Cursor};

pub struct V1_20_4;

impl PacketReadHandler for V1_20_4 {
    fn handle_packet_read(
        server: &mut Server,
        player: &mut Player,
        value: &mut Cursor<Vec<u8>>,
    ) -> Result<()> {
        let packet_id = value.read_var_i32()?;
        let connection_state = &player.session_relay.connection_state;
        let bytes = value;
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
            (ConnectionState::Login, 3) => {
                LoginAcknowledged::try_from(bytes)?.handle_packet(server, player)?
            }
            (ConnectionState::Confgiuration, 0x01) => {
                PluginMessage::try_from(bytes)?.handle_packet(server, player)?
            }
            (ConnectionState::Confgiuration, 0x00) => {
                ClientInformation::try_from(bytes)?.handle_packet(server, player)?
            }
            (ConnectionState::Confgiuration, 0x02) => {
                FinishConfiguration::try_from(bytes)?.handle_packet(server, player)?
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("{:?}[{:#04X?}] not exists", connection_state, packet_id),
                ))
            }
        };
        Ok(())
    }
}
