use std::io::{prelude::Read, Cursor, Error, Result, Write};

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, U8Read, U8Write},
    net::prelude::{PacketHandler, PacketWriter, Socket},
    protocol::v1_20_4::configuration::{
        feature_flags::FeatureFlags,
        finish_configuration::FinishConfigurationS2c,
        registry::{
            Biome, ChatType, DamageType, Decoration, DimensionType, Effects, IntegerDistribution,
            MonsterSpawnLightLevel, Registry, RegistryData, RegistryEntry,
        },
    },
    server::prelude::{ConnectionState, GamePlayer, GameServer},
};

pub struct LoginAcknowledged {}

impl Encoder for LoginAcknowledged {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Decoder for LoginAcknowledged {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(LoginAcknowledged {})
    }
}

impl PacketHandler<GameServer> for LoginAcknowledged {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        player.session_relay.connection_state = ConnectionState::Confgiuration;

        let registry_data = RegistryData {};
        registry_data.send_packet(player)?;
        let feature_flags = FeatureFlags {
            feature_flags: Vec::new(),
        };
        feature_flags.send_packet(player)?;
        let finish_configuration = FinishConfigurationS2c;
        finish_configuration.send_packet(player)?;
        Ok(())
    }
}
