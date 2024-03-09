use std::io::{prelude::Write, Result};

use uuid::Uuid;

use crate::{
    io::prelude::{
        Encoder, F64Write, Identifier, U8Write, UuidWrite, VarIntSizedVecWrite, VarIntWrite,
    },
    net::prelude::{PacketId, Socket},
    server::prelude::LoginPlayer,
};

pub struct UpdateAttributes {
    pub entity_id: i32,
    pub properties: Vec<AttributeProperty>,
}

impl Encoder for UpdateAttributes {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.entity_id)?;
        writer.write_var_int_sized_vec(&self.properties)?;
        Ok(())
    }
}

impl PacketId<LoginPlayer> for UpdateAttributes {
    fn get_packet_id(&self, player: &mut Socket<LoginPlayer>) -> Result<i32> {
        Ok(0x71)
    }
}

pub struct AttributeProperty {
    pub key: Identifier,
    pub value: f64,
    pub modifiers: Vec<ModifierData>,
}

impl Encoder for AttributeProperty {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.key.encode_to_write(writer)?;
        writer.write_f64(self.value)?;
        writer.write_var_int_sized_vec(&self.modifiers)?;
        Ok(())
    }
}

pub struct ModifierData {
    pub uuid: Uuid,
    pub amount: f64,
    pub operation: ModifierOperation,
}

impl Encoder for ModifierData {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_uuid(self.uuid)?;
        writer.write_f64(self.amount)?;
        self.operation.encode_to_write(writer)?;
        Ok(())
    }
}

pub enum ModifierOperation {
    Add,
    Precentage,
    Multiply,
}

impl Encoder for ModifierOperation {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(match self {
            ModifierOperation::Add => 0,
            ModifierOperation::Precentage => 1,
            ModifierOperation::Multiply => 2,
        })?;
        Ok(())
    }
}
