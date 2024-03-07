use crate::{
    io::{
        array::VarIntSizedVecWrite,
        encoding::{Decoder, Encoder},
        identifier::{Identifier, ReadIdentifier},
        primitives::{I64Write, U8Write, WriteBool},
        var_int::VarIntWrite,
    },
    net::prelude::{PacketIdentnifier, Player}, server::prelude::GameMode,
};

use crate::server::{
    coordinates::Position,
};

pub struct LoginPlay {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_names: Vec<Identifier>,
    pub max_players: i32,
    pub view_distance: i32,
    pub simulation_distance: i32,
    pub reduce_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: Identifier,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub game_mode: GameMode,
    pub previous_game_mode: Option<GameMode>,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<DeathLocation>,
    pub portal_cooldown: i32,
}

pub struct DeathLocation {
    pub death_dimension_name: Identifier,
    pub death_location: Position,
}

impl Encoder for DeathLocation {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.death_dimension_name.encode_to_write(writer)?;
        self.death_location.encode_to_write(writer)?;
        Ok(())
    }
}

impl Decoder for DeathLocation {
    fn decode_from_read<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(DeathLocation {
            death_dimension_name: reader.read_identifier()?,
            death_location: Position::decode_from_read(reader)?,
        })
    }
}

impl Encoder for LoginPlay {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&i32::to_be_bytes(self.entity_id))?;
        writer.write_bool(self.is_hardcore)?;
        writer.write_var_int_sized_vec(&self.dimension_names)?;
        writer.write_var_i32(self.max_players)?;
        writer.write_var_i32(self.view_distance)?;
        writer.write_var_i32(self.simulation_distance)?;
        writer.write_bool(self.reduce_debug_info)?;
        writer.write_bool(self.enable_respawn_screen)?;
        writer.write_bool(self.do_limited_crafting)?;
        self.dimension_type.encode_to_write(writer)?;
        self.dimension_name.encode_to_write(writer)?;
        writer.write_i64(self.hashed_seed)?;
        self.game_mode.encode_to_write(writer)?;
        if let Some(previous_game_mode) = &self.previous_game_mode {
            previous_game_mode.encode_to_write(writer)?;
        } else {
            writer.write_var_i32(0)?;
        }
        writer.write_bool(self.is_debug)?;
        writer.write_bool(self.is_flat)?;
        if let Some(death_location) = &self.death_location {
            death_location.encode_to_write(writer)?;
        } else {
            writer.write_u8(0)?;
        }
        writer.write_var_i32(self.portal_cooldown)?;
        Ok(())
    }
}

impl PacketIdentnifier for LoginPlay {
    fn get_packet_id(&self, player: &mut Player) -> std::io::Result<i32> {
        Ok(0x29)
    }
}
