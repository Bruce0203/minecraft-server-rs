use std::io::{Cursor, Result, Write};

use flate2::{write::ZlibEncoder, Compression};
use mc_io::{encoding::Encoder, var_int::VarIntWrite};
use socket_selector::Socket;

use crate::server::prelude::Player;

pub trait PacketWriter: Sized + Encoder {
    fn get_packet_id(&self, player: &mut Player) -> Result<i32>;

    fn send_packet(&self, player: &mut Player) -> Result<()> {
        let id_and_payload_bytes = self.write_packet_id_and_payload(player)?;
        let compression_threshold = player.session_relay.compression_threshold;

        let mut result_buf =
            Cursor::new(Vec::with_capacity(id_and_payload_bytes.get_ref().len() + 6));

        let id_and_payload_bytes_len = id_and_payload_bytes.get_ref().len() as i32;
        if compression_threshold == -1 {
            result_buf.write_var_i32(id_and_payload_bytes_len).unwrap();
            result_buf
                .write_all(id_and_payload_bytes.get_ref())
                .unwrap();
        } else {
            if compression_threshold > id_and_payload_bytes_len {
                result_buf.write_var_i32(id_and_payload_bytes_len + 1)?;
                result_buf.write_all(&[0x00])?;
                result_buf.write_all(id_and_payload_bytes.get_ref())?;
            } else {
                let mut encoder = ZlibEncoder::new(Cursor::new(Vec::new()), Compression::default());
                encoder.write_all(id_and_payload_bytes.get_ref())?;
                let compressed_bytes = encoder.finish()?;
                let mut data = Cursor::new(Vec::new());
                data.write_var_i32(id_and_payload_bytes_len)?;
                data.write_all(compressed_bytes.get_ref())?;
                result_buf.write_var_i32(data.get_ref().len() as i32)?;
                result_buf.write_all(data.get_ref())?;
            }
        }
        let result = result_buf.into_inner();
        player.get_stream().write_all(&result)?;
        Ok(())
    }

    fn write_packet_id_and_payload(&self, player: &mut Player) -> Result<Cursor<Vec<u8>>> {
        let payload = self.encode()?;
        let packet_id = self.get_packet_id(player)?;

        let mut payload_buf = Cursor::new(Vec::with_capacity(payload.get_ref().len() + 5));

        payload_buf.write_var_i32(packet_id)?;
        payload_buf.write(payload.get_ref())?;
        let payload_bytes = payload_buf.into_inner();
        Ok(Cursor::new(payload_bytes))
    }
}
