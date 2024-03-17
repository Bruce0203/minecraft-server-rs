use crate::server::prelude::{GamePlayer, GameServer};

use super::{
    macros::{protocol, protocol_server},
    v1_20_4::v1_20_4::V1_20_4,
};

pub struct Minecraft;

protocol_server!(GameServer, GamePlayer, V1_20_4, V1_20_4,);
