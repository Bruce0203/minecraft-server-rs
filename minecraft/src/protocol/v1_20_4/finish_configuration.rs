use std::io::{Cursor, Error, Result, Write};

use common_server::{encoding::Encoder, packet::PacketHandler};

use crate::{
    connection::{packet_writer::PacketWriter, player::Player, ConnectionState},
    protocol::v1_20_4::login_play::LoginPlay,
    server::Server,
};

pub struct FinishConfiguration {}

impl FinishConfiguration {
    pub fn new() -> FinishConfiguration {
        FinishConfiguration {}
    }
}

impl TryFrom<&mut Cursor<Vec<u8>>> for FinishConfiguration {
    type Error = Error;

    fn try_from(value: &mut Cursor<Vec<u8>>) -> Result<Self> {
        Ok(FinishConfiguration {})
    }
}

impl PacketHandler<Server, Player> for FinishConfiguration {
    fn handle_packet(&self, server: &mut Server, player: &mut Player) -> Result<()> {
        println!("configuration finished");
        player.session_relay.connection_state = ConnectionState::Play;
        let login_play = LoginPlay {
            entity_id: todo!(),
            is_hardcore: todo!(),
            dimension_count: todo!(),
            dimension_names: todo!(),
            max_players: todo!(),
            view_distance: todo!(),
            simulation_distance: todo!(),
            reduce_debug_info: todo!(),
            enable_respawn_screen: todo!(),
            do_limited_crafting: todo!(),
            dimension_type: todo!(),
            dimension_name: todo!(),
            hashed_seed: todo!(),
            game_mode: todo!(),
            previous_game_mode: todo!(),
            is_debug: todo!(),
            is_flat: todo!(),
            has_death_location: todo!(),
            death_dimension_name: todo!(),
            death_location: todo!(),
            portal_cooldown: todo!(),
        };
        Ok(())
    }
}

impl PacketWriter for FinishConfiguration {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32> {
        Ok(0x02)
    }
}

impl Encoder for FinishConfiguration {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(())
    }
}
