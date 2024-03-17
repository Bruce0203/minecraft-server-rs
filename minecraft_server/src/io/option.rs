use std::{
    io::{Read, Result, Write},
    ops::Deref,
};

use serde::de::value;

use crate::server::prelude::Chat;

use super::{
    encoding::{Decoder, Encoder},
    prelude::{Buffer, DecoderDeref, EncoderDeref, VarInt},
    primitive::{BoolRead, U8Write, WriteBool},
};

pub trait OptionRead {
    fn read_option<D: Decoder>(&mut self) -> Result<Option<D>>;
}

pub trait OptionWrite {
    fn write_option<E: Encoder>(&mut self, value: &Option<E>) -> Result<()>;
}

impl<R: Read> OptionRead for R {
    fn read_option<D: Decoder>(&mut self) -> Result<Option<D>> {
        Ok(if self.read_bool()? {
            Some(D::decode_from_read(self)?)
        } else {
            None
        })
    }
}

impl OptionWrite for Buffer {
    fn write_option<E: Encoder>(&mut self, value: &Option<E>) -> Result<()> {
        if let Some(encoder) = value {
            self.write_bool(true)?;
            encoder.encode_to_buffer(self)?;
        } else {
            self.write_bool(false)?;
        }
        Ok(())
    }
}

impl<T: Encoder> !EncoderDeref for Option<T> {}

impl<T: Encoder> EncoderDeref for Box<T> {}

impl<T: Encoder> Encoder for Option<T> {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        match self {
            Some(value) => {
                buf.write_u8(1)?;
                T::encode_to_buffer(value, buf)?;
                Ok(())
            }
            None => {
                buf.write_u8(0)?;
                Ok(())
            }
        }
    }
}

impl<T: Decoder> !DecoderDeref for Option<T> {}

impl<T: Decoder> Decoder for Option<T> {
    fn decode_from_read<R: Read>(reader: &mut R) -> Result<Self> {
        Ok(Some(T::decode_from_read(reader)?))
    }
}

#[test]
fn encode_option() {
    let opt = Some(VarInt(0));
    let mut box_opt = Box::new(opt.clone());
    let mut box_opt = &Box::new(opt.clone());
    let mut opt_box = Some(Box::new(VarInt(0)));
    let mut opt_box = &Some(Box::new(VarInt(0)));
    let mut buf = Buffer::new(vec![]);
    opt.encode_to_buffer(&mut buf).unwrap();
    opt_box.encode_to_buffer(&mut buf).unwrap();
    box_opt.encode_to_buffer(&mut buf).unwrap();
    let chat = &Some(Box::new(Chat::from("".to_string())));
    chat.encode_to_buffer(&mut buf).unwrap();
}
