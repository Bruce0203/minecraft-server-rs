use crate::io::prelude::Decoder;

#[derive(Debug)]
pub struct SoundEffect {}

impl Decoder for SoundEffect {
    fn decode_from_read(reader: &mut crate::io::prelude::Buffer) -> std::io::Result<Self> {
        //TODO wip
        Ok(SoundEffect {})
    }
}
