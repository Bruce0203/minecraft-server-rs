use crate::{
    protocol::protocol_macro::{packets, protocols},
    server::prelude::{LoginPlayer, LoginServer},
};

use super::{
    configuration::{
        client_information::ClientInformation, finish_configuration::FinishConfiguration,
        plugin_message::PluginMessage,
    },
    login::{
        login_acknowledged::LoginAcknowledged, login_start::LoginStart, login_success::LoginSuccess,
    },
    status::{ping::PingRequest, status::{StatusRequest, StatusResponse}},
};
use crate::net::prelude::ConnectionState::*;

pub struct V1_20_4;

packets!(
    LoginServer,
    LoginPlayer,
    V1_20_4,
    765,
    (Bound::Server, HandShake, 0x00, super::handshake::HandShake),
    (Bound::Server, Status, 0x00, StatusRequest),
    (Bound::Server, Status, 0x01, PingRequest),
    (Bound::Server, Login, 0x00, LoginStart),
    (Bound::Client, Login, 0x03, LoginAcknowledged),
    (Bound::Server, Confgiuration, 0x00, ClientInformation),
    (Bound::Server, Confgiuration, 0x01, PluginMessage),
    (Bound::Server, Confgiuration, 0x02, FinishConfiguration),
);
