use crate::{
    protocol::macros::{protocols, receiving_packets, sending_packets},
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
    status::{PingRequest, PingResponse, StatusRequest, StatusResponse},
};
use crate::net::prelude::ConnectionState::*;

pub struct V1_20_4;
receiving_packets!(
    V1_20_4,
    (0x00, HandShake, super::handshake::HandShake),
    (0x00, Status, StatusRequest),
    (0x01, Status, PingRequest),
    (0x00, Login, LoginStart),
    (0x03, Login, LoginAcknowledged),
    (0x00, Confgiuration, ClientInformation),
    (0x01, Confgiuration, PluginMessage),
    (0x02, Confgiuration, FinishConfiguration),
);

sending_packets!(V1_20_4,
    (0x00, Status, StatusResponse<'_>),
    (0x01, Status, PingResponse),
);
