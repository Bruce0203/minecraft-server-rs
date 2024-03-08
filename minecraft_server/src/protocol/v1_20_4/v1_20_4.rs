use std::io::{Error, ErrorKind};

use crate::{
    io::prelude::{Decoder, VarIntRead},
    net::prelude::{ConnectionState, PacketHandler, Player},
    protocol::packets,
};

use super::{
    client_information::ClientInformation, finish_configuration::FinishConfiguration,
    handshake::HandShake, login_acknowledged::LoginAcknowledged, login_start::LoginStart,
    ping::PingRequest, plugin_message::PluginMessage, status::StatusRequest,
};
use crate::protocol;

pub struct V1_20_4;
use protocol::Bound::*;
packets!(
    V1_20_4,
    (Server, ConnectionState::HandShake, 0x00, HandShake),
    (Server, ConnectionState::Status, 0x00, StatusRequest),
    (Server, ConnectionState::Status, 0x01, PingRequest),
    (Server, ConnectionState::Login, 0x00, LoginStart),
    (Server, ConnectionState::Login, 0x03, LoginAcknowledged),
    (Server, ConnectionState::Confgiuration, 0x00, ClientInformation),
    (Server, ConnectionState::Confgiuration, 0x01, PluginMessage),
    (Server, ConnectionState::Confgiuration, 0x02, FinishConfiguration),
);
