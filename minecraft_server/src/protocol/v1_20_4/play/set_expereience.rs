use crate::io::prelude::{Decoder, F32Read, VarIntRead};

#[derive(Debug)]
pub struct SetExperience {
    experience_bar: f32,
    level: i32,
    total_experience: i32,
}

impl Decoder for SetExperience {
    fn decode_from_read(reader: &mut crate::io::prelude::Buffer) -> std::io::Result<Self> {
        Ok(SetExperience {
            experience_bar: reader.read_f32()?,
            level: reader.read_var_i32()?,
            total_experience: reader.read_var_i32()?,
        })
    }
}
