use crate::protocol;
use crate::server::prelude::GamePlayer;
use crate::{
    io::prelude::{Decoder, VarIntRead},
    net::prelude::{ConnectionState, PacketHandler},
    protocol::protocol_macro::packets,
};
use std::io::{Error, ErrorKind};
use ConnectionState::*;

use super::status::ping::PingRequest;
use super::status::status::StatusRequest;

pub struct V1_20_4;

packets!(
    Server,
    GamePlayer,
    V1_20_4,
    (Bound::Server, HandShake, 0x00, super::handshake::HandShake),
    (Bound::Server, Status, 0x00, StatusRequest),
    (Bound::Server, Status, 0x01, PingRequest),
    //    (Bound::Server, Login, 0x00, LoginStart),
    //    (Bound::Server, Login, 0x03, LoginAcknowledged),
    //
    //    (Bound::Server, Confgiuration, 0x00, ClientInformation),
    //    (Bound::Server, Confgiuration, 0x01, PluginMessage),
    //    (Bound::Server, Confgiuration, 0x02, FinishConfiguration),
);
