use std::{collections::BTreeMap, io::Result, option::Option};

use crate::io::prelude::{Buffer, Encoder, VarIntWrite};

use super::metadata::prelude::BlockState;

pub struct Palette {
    id_to_state: BTreeMap<u32, BlockState>,
    state_to_id: BTreeMap<BlockState, u32>,
    bits_per_block: u8,
}

impl Palette {
    pub fn new(bits_per_block: u8) -> Palette {
        Palette {
            id_to_state: BTreeMap::new(),
            state_to_id: BTreeMap::new(),
            bits_per_block,
        }
    }

    fn id_for_state(&self, state: &BlockState) -> u32 {
        *self.state_to_id.get(state).unwrap()
    }

    fn state_for_id(&self, id: u32) -> BlockState {
        *self.id_to_state.get(&id).unwrap()
    }
}

impl Encoder for Palette {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_var_i32(self.id_to_state.len() as i32)?;
        for state in self.id_to_state.values() {
            let state_id = get_global_palette_idfrom_state(state);
            buf.write_var_i32(state_id as i32)?;
        }
        Ok(())
    }
}

pub fn choose_palette(bits_per_block: u8) -> Palette {
    if bits_per_block <= 4 {
        Palette::new(4)
    } else {
        Palette::new(bits_per_block)
    }
}

fn get_global_palette_idfrom_state(state: &BlockState) -> u32 {
    (state.0 as u32)
}

fn get_state_from_global_palette_id<'a>(id: u32) -> BlockState {
    todo!()
}
