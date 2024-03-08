use std::io::{Error, ErrorKind};

use crate::{
    io::prelude::{Decoder, VarIntRead},
    net::prelude::{ConnectionState, PacketHandler, Player},
    server::prelude::Server,
};

use super::{
    client_information::ClientInformation, finish_configuration::FinishConfiguration,
    handshake::HandShake, login_acknowledged::LoginAcknowledged, login_start::LoginStart,
    ping::PingRequest, plugin_message::PluginMessage, status::StatusRequest,
};
use crate::protocol;

pub struct V1_20_4;
protocol::packets!(
    V1_20_4,
    (ConnectionState::HandShake, 0x00, HandShake),
    (ConnectionState::Status, 0x00, StatusRequest),
    (ConnectionState::Status, 0x01, PingRequest),
    (ConnectionState::Login, 0x00, LoginStart),
    (ConnectionState::Login, 0x03, LoginAcknowledged),
    (ConnectionState::Confgiuration, 0x00, ClientInformation),
    (ConnectionState::Confgiuration, 0x01, PluginMessage),
    (ConnectionState::Confgiuration, 0x02, FinishConfiguration),
);
