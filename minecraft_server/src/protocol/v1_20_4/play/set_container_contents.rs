use std::io::{prelude::Write, Result};

use crate::{
    io::prelude::{Encoder, U8Write, VarIntSizedVecWrite, VarIntWrite},
    server::slot::{self, Slot},
};

pub struct SetContainerContent {
    pub window_id: u8,
    pub state_id: i32,
    pub slot_data: Vec<Slot>,
    pub carried_item: Slot,
}

impl Encoder for SetContainerContent {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_u8(self.window_id)?;
        buf.write_var_i32(self.state_id)?;
        buf.write_var_int_sized_vec(&self.slot_data)?;
        self.carried_item.encode_to_buffer(buf)?;
        Ok(())
    }
}
