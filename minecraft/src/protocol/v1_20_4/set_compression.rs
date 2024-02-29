use std::io::{Result, Write};

use bytes::{BufMut, BytesMut};
use common_server::encoding::Encoder;
use common_server::packet::PacketHandler;
use common_server::selector::Socket;
use common_server::var_int::VarIntWrite;
use flate2::write::ZlibDecoder;
use flate2::{write::ZlibEncoder, Compression};

use crate::{connection::packet_writer::PacketWriter, prelude::Player};

#[test]
fn test_flate2() {
    let model = "hello world";
    let bytes = &mut BytesMut::new();
    let mut e = ZlibEncoder::new(bytes.writer(), Compression::default());
    e.write_all("hello world".as_bytes()).unwrap();
    let compressed = e.finish().unwrap();
    println!("{:?}", &bytes);
    let decompressed_bytes = &mut BytesMut::new();
    let mut d = ZlibDecoder::new(decompressed_bytes.writer());
    d.write_all(&bytes).unwrap();
    let finish = d.finish().unwrap();
    println!("{:?}", &decompressed_bytes);
    println!("{:?}", String::from_utf8(decompressed_bytes.to_vec()));
}

pub struct SetCompression {
    pub compression_threshold: i32,
}

impl Encoder for SetCompression {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_var_i32(self.compression_threshold)?;
        Ok(())
    }
}

impl PacketWriter for SetCompression {
    fn get_packet_id(&self, _socket: &mut Player) -> Result<i32> {
        Ok(0x03)
    }
}

pub fn set_compression(socket: &mut Player, compression_threshold: i32) -> Result<()> {
    let set_compression = SetCompression {
        compression_threshold,
    };
    set_compression.send_packet(socket)?;
    socket.session_relay.compression_threshold = compression_threshold;
    Ok(())
}
