use std::io::{Cursor, Error, Result, Write};

use mc_io::{encoding::Encoder, identifier::ToIdentifier};

use crate::{
    connection::prelude::{ConnectionState, PacketHandler, PacketWriter},
    protocol::v1_20_4::{
        login_play::LoginPlay,
        player_abilities::{PlayerAbilities, PlayerAbility},
        set_default_position::SetDefaultPosition,
        set_held_item::SetHeldItem, server_data::ServerData,
    },
    server::prelude::*,
};

pub struct FinishConfiguration {}

impl FinishConfiguration {
    pub fn new() -> FinishConfiguration {
        FinishConfiguration {}
    }
}

impl TryFrom<&mut Cursor<Vec<u8>>> for FinishConfiguration {
    type Error = Error;

    fn try_from(_value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(FinishConfiguration {})
    }
}

impl PacketHandler<Server, Player> for FinishConfiguration {
    fn handle_packet(&self, _server: &mut Server, player: &mut Player) -> Result<()> {
        println!("configuration finished");
        player.session_relay.connection_state = ConnectionState::Play;
        let login_play = LoginPlay {
            entity_id: 0,
            is_hardcore: false,
            dimension_names: vec!["minecraft:overworld".to_identifier()],
            max_players: 20,
            view_distance: 32,
            simulation_distance: 32,
            reduce_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: "minecraft:overworld".to_identifier(),
            dimension_name: "minecraft:overworld".to_identifier(),
            hashed_seed: 0,
            game_mode: GameMode::Creative,
            previous_game_mode: None,
            is_debug: false,
            is_flat: true,
            death_location: None,
            portal_cooldown: 0,
        };
        login_play.send_packet(player)?;
        let player_abilities = PlayerAbilities {
            flags: PlayerAbility::Flying,
            flying_speed: 0.5,
            field_of_view_modifier: 0.1,
        };
        player_abilities.send_packet(player)?;
        SetHeldItem { slot: 0 }.send_packet(player)?;
        SetDefaultPosition {
            location: Position::new(0, 0, 0),
            angle: 0.0,
        }
        .send_packet(player)?;
        ServerData {
            message_of_the_day: "MC sv".to_string(),
            icon: None,
            enforce_secure_chat: true,
        }.send_packet(player)?;
        Ok(())
    }
}

impl PacketWriter for FinishConfiguration {
    fn get_packet_id(&self, _player: &mut Player) -> Result<i32> {
        Ok(0x02)
    }
}

impl Encoder for FinishConfiguration {
    fn encode_to_write<W: Write>(&self, _writer: &mut W) -> Result<()> {
        Ok(())
    }
}
