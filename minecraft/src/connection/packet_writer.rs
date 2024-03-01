use std::io::{Result, Write};

use bytes::{BufMut, BytesMut};
use common_server::{encoding::Encoder, selector::Socket, var_int::VarIntWrite};
use flate2::{write::ZlibEncoder, Compression};

use crate::prelude::Player;

pub trait PacketWriter: Sized + Encoder {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32>;

    fn send_packet(&self, player: &mut Player) -> Result<()> {
        let id_and_payload_bytes = self.write_packet_id_and_payload(player)?;
        let compression_threshold = player.session_relay.compression_threshold;

        let result_buf = BytesMut::with_capacity(id_and_payload_bytes.len() + 6);
        let mut result_buf_writer = result_buf.writer();

        let id_and_payload_bytes_len = id_and_payload_bytes.len() as i32;
        if compression_threshold == -1 {
            result_buf_writer
                .write_var_i32(id_and_payload_bytes_len)
                .unwrap();
            result_buf_writer.write_all(&id_and_payload_bytes).unwrap();
        } else {
            if compression_threshold > id_and_payload_bytes_len {
                result_buf_writer
                    .write_var_i32(id_and_payload_bytes_len + 1)
                    .unwrap();
                result_buf_writer.write_all(&[0x00]).unwrap();
                result_buf_writer.write_all(&id_and_payload_bytes).unwrap();
            } else {
                let compressed_bytes = &mut BytesMut::new();
                let mut encoder =
                    ZlibEncoder::new(compressed_bytes.writer(), Compression::default());
                encoder.write_all(&id_and_payload_bytes).unwrap();
                encoder.finish().unwrap();
                result_buf_writer
                    .write_var_i32(compressed_bytes.len() as i32 + 1)
                    .unwrap();
                result_buf_writer.write_all(&[0x00]).unwrap();
                result_buf_writer.write_all(&compressed_bytes).unwrap();
            }
        }
        let result = result_buf_writer.into_inner();
        player.stream().write_all(&result)?;
        Ok(())
    }

    fn write_packet_id_and_payload(&self, player: &mut Player) -> Result<BytesMut> {
        let payload = self.encode()?;
        let packet_id = self.get_packet_id(player)?;

        let payload_buf = BytesMut::with_capacity(payload.len() + 5);
        let mut payload_writer = payload_buf.writer();

        payload_writer.write_var_i32(packet_id)?;
        payload_writer.write(&payload)?;
        let payload_bytes = payload_writer.into_inner();
        Ok(payload_bytes)
    }
}
