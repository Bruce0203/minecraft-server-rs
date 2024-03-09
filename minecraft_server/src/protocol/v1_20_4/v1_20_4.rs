use crate::{
    protocol::protocol_macro::{packets, protocols},
    server::prelude::{LoginPlayer, LoginServer},
};

use super::status::{ping::PingRequest, status::StatusRequest};
use crate::net::prelude::ConnectionState::*;

#[derive(derive_more::Constructor)]
pub struct HandShakeServer;
#[derive(Default)]
pub struct HandShakingPlayer;
pub struct HandShakeProtocol;
protocols!(100, HandShakeServer, HandShakingPlayer, HandShakeProtocol,);
packets!(
    HandShakeServer,
    HandShakingPlayer,
    HandShakeProtocol,
    0,
    (Bound::Server, HandShake, 0x00, super::handshake::HandShake),
);

pub struct V1_20_4;

packets!(
    LoginServer,
    LoginPlayer,
    V1_20_4,
    765,
    (Bound::Server, Status, 0x00, StatusRequest),
    (Bound::Server, Status, 0x01, PingRequest),
    //    (Bound::Server, Login, 0x00, LoginStart),
    //    (Bound::Server, Login, 0x03, LoginAcknowledged),
    //
    //    (Bound::Server, Confgiuration, 0x00, ClientInformation),
    //    (Bound::Server, Confgiuration, 0x01, PluginMessage),
    //    (Bound::Server, Confgiuration, 0x02, FinishConfiguration),
);
