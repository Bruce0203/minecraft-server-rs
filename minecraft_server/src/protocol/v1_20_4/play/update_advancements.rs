use crate::io::prelude::{Decoder, Encoder};

#[derive(Debug)]
pub struct UpdateAdvancements {}

impl Decoder for UpdateAdvancements {
    fn decode_from_read(reader: &mut crate::io::prelude::Buffer) -> std::io::Result<Self> {
        Ok(UpdateAdvancements {})
    }
}
