use std::io::{Read, Write};

use crate::{
    io::prelude::{
        BoolRead, Buffer, Decoder, Encoder, I32Read, I64Read, I64Write, Identifier, IdentifierRead,
        OptionRead, U8Write, VarIntRead, VarIntSizedVecRead, VarIntSizedVecWrite, VarIntWrite,
        WriteBool,
    },
    net::prelude::{PacketId, Socket},
    server::{
        coordinates::Position,
        prelude::{GameMode, GamePlayer},
    },
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct DeathLocation {
    pub death_dimension_name: Identifier,
    pub death_location: Position,
}

impl Encoder for DeathLocation {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> std::io::Result<()> {
        self.death_dimension_name.encode_to_buffer(buf)?;
        self.death_location.encode_to_buffer(buf)?;
        Ok(())
    }
}

impl Decoder for DeathLocation {
    fn decode_from_read(reader: &mut Buffer) -> std::io::Result<Self> {
        Ok(DeathLocation {
            death_dimension_name: reader.read_identifier()?,
            death_location: Position::decode_from_read(reader)?,
        })
    }
}

impl Encoder for LoginPlay {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> std::io::Result<()> {
        buf.write_all(&i32::to_be_bytes(self.entity_id))?;
        buf.write_bool(self.is_hardcore)?;
        buf.write_var_int_sized_vec(&self.dimension_names)?;
        buf.write_var_i32(self.max_players)?;
        buf.write_var_i32(self.view_distance)?;
        buf.write_var_i32(self.simulation_distance)?;
        buf.write_bool(self.reduce_debug_info)?;
        buf.write_bool(self.enable_respawn_screen)?;
        buf.write_bool(self.do_limited_crafting)?;
        self.dimension_type.encode_to_buffer(buf)?;
        self.dimension_name.encode_to_buffer(buf)?;
        buf.write_i64(self.hashed_seed)?;
        self.game_mode.encode_to_buffer(buf)?;
        if let Some(previous_game_mode) = &self.previous_game_mode {
            previous_game_mode.encode_to_buffer(buf)?;
        } else {
            buf.write_var_i32(0)?;
        }
        buf.write_bool(self.is_debug)?;
        buf.write_bool(self.is_flat)?;
        if let Some(death_location) = &self.death_location {
            death_location.encode_to_buffer(buf)?;
        } else {
            buf.write_u8(0)?;
        }
        buf.write_var_i32(self.portal_cooldown)?;
        Ok(())
    }
}

impl Decoder for LoginPlay {
    fn decode_from_read(reader: &mut Buffer) -> std::io::Result<Self> {
        Ok(LoginPlay {
            entity_id: reader.read_i32()?,
            is_hardcore: reader.read_bool()?,
            dimension_names: reader.read_var_int_sized_vec()?,
            max_players: reader.read_var_i32()?,
            view_distance: reader.read_var_i32()?,
            simulation_distance: reader.read_var_i32()?,
            reduce_debug_info: reader.read_bool()?,
            enable_respawn_screen: reader.read_bool()?,
            do_limited_crafting: reader.read_bool()?,
            dimension_type: reader.read_identifier()?,
            dimension_name: reader.read_identifier()?,
            hashed_seed: reader.read_i64()?,
            game_mode: GameMode::decode_from_read(reader)?,
            previous_game_mode: reader.read_option()?,
            is_debug: reader.read_bool()?,
            is_flat: reader.read_bool()?,
            death_location: reader.read_option()?,
            portal_cooldown: reader.read_var_i32()?,
        })
    }
}
