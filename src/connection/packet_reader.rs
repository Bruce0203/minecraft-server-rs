use std::io::{Cursor, Error, ErrorKind, Read, Write};

use flate2::write::ZlibDecoder;
use mc_io::var_int::VarIntRead;
use serde::de::value;

use super::SessionRelay;

pub(super) fn read_packet_id_and_payload(
    value: &mut Cursor<Vec<u8>>,
    session_relay: &mut SessionRelay,
) -> std::io::Result<Cursor<Vec<u8>>> {
    let packet_len = value.read_var_i32()?;
    let mut buf = Vec::with_capacity(packet_len as usize);
    value.take(packet_len as u64).read_to_end(&mut buf)?;
    let mut value = Cursor::new(buf);
    let packet_body_len = value.get_ref().len() - value.position() as usize;
    if packet_body_len < packet_len as usize {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "actual packet length is short than enough",
        ));
    }
    let compression_threshold = session_relay.compression_threshold;
    if compression_threshold == -1 {
        return Ok(value);
    } else {
        let decompressed_len = value.read_var_i32()?;
        if compression_threshold as usize > packet_body_len {
            Ok(value)
        } else {
            println!("start decompressing");
            println!("{}", decompressed_len);
            let decompressed_bytes = Cursor::new(Vec::new());
            let mut d = ZlibDecoder::new(decompressed_bytes);
            d.write_all(value.get_ref())?;
            let finish = d.finish()?;
            Ok(finish)
        }
    }
}
