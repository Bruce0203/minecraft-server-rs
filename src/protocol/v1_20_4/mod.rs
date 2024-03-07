pub mod client_information;
pub mod feature_flags;
pub mod finish_configuration;
pub mod handshake;
pub mod login_acknowledged;
pub mod login_play;
pub mod login_start;
pub mod login_success;
pub mod ping;
pub mod player_abilities;
pub mod player_info;
pub mod plugin_message;
pub mod registry;
pub mod server_data;
pub mod set_compression;
pub mod set_default_position;
pub mod set_held_item;
pub mod status;

use crate::io::var_int::VarIntRead;

use crate::server::prelude::*;

use self::{
    client_information::ClientInformation, finish_configuration::FinishConfiguration,
    handshake::HandShake, login_acknowledged::LoginAcknowledged, login_start::LoginStart,
    ping::PingRequest, plugin_message::PluginMessage, status::StatusRequest,
};
use std::io::{Error, ErrorKind, Result};

use super::prelude::{ConnectionState, PacketHandler, Decoder};

pub struct V1_20_4;

//todo macro this things 
impl PacketHandler for V1_20_4 {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> Result<()> {
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
