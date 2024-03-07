use std::io::{Error, ErrorKind};

use crate::{
    io::prelude::{Decoder, VarIntRead},
    net::prelude::{PacketHandler, Player},
    protocol::prelude::ConnectionState,
    server::prelude::Server,
};

use super::{
    client_information::ClientInformation, finish_configuration::FinishConfiguration,
    handshake::HandShake, login_acknowledged::LoginAcknowledged, login_start::LoginStart,
    ping::PingRequest, plugin_message::PluginMessage, status::StatusRequest,
};

pub struct V1_20_4;

impl PacketHandler for V1_20_4 {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> std::io::Result<()> {
        let bytes = &mut player.packet_buf;
        let packet_id = bytes.read_var_i32()?;
        let connection_state = &player.session_relay.connection_state;
        match (connection_state, packet_id) {
            (ConnectionState::HandShake, 0) => {
                HandShake::decode_from_read(bytes)?.handle_packet(server, player)?
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
