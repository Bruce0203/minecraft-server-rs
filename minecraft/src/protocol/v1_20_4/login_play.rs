use common_server::{
    array::VarIntSizedVecWrite,
    encoding::Encoder,
    identifier::Identifier,
    primitives::{I64Write, WriteBool},
    var_int::VarIntWrite,
};

use crate::server::{game_mode::GameMode, position::Position};

pub struct LoginPlay {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimension_count: i32,
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
    pub has_death_location: bool,
    pub death_dimension_name: Option<Identifier>,
    pub death_location: Option<Position>,
    pub portal_cooldown: i32,
}

impl Encoder for LoginPlay {
    fn encode_to_write<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&i32::to_be_bytes(self.entity_id))?;
        writer.write_bool(self.is_hardcore)?;
        writer.write_var_i32(self.dimension_count)?;
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
        writer.write_bool(self.has_death_location)?;
        if let Some(death_dimension_name) = &self.death_dimension_name {
            death_dimension_name.encode_to_write(writer)?;
        } else {
            writer.write_var_i32(0)?;
        }
        if let Some(death_location) = &self.death_location {
            todo!()
        } else {
            writer.write_var_i32(0)?;
        }
        writer.write_var_i32(self.portal_cooldown)?;
        Ok(())
    }
}
