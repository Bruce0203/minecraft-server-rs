use crate::server::prelude::{GamePlayer, GameServer};

use super::{
    macros::{protocol, protocol_server},
    v1_20_4::v1_20_4::MinecraftServerV1_20_4,
};

protocol_server!(
    GameServer,
    GamePlayer,
    MinecraftServerV1_20_4,
    MinecraftServerV1_20_4,
);
