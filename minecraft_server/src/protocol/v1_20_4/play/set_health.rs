use std::io::Result;

use crate::io::prelude::{Buffer, Encoder, F32Write, VarIntWrite};

pub struct SetHealth {
    pub health: f32,
    pub food: i32,
    pub food_saturation: f32,
}

impl Encoder for SetHealth {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        buf.write_f32(self.health)?;
        buf.write_var_i32(self.food)?;
        buf.write_f32(self.food_saturation)?;
        Ok(())
    }
}
