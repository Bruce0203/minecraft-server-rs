use std::io::{Cursor, Error, ErrorKind, Write};

use common_server::var_int::VarIntRead;
use flate2::write::ZlibDecoder;

use super::SessionRelay;

pub(super) fn read_packet_id_and_payload( buf: &[u8],
    session_relay: &mut SessionRelay,
) -> std::io::Result<Cursor<Vec<u8>>> {
    let mut value = Cursor::new(buf.to_vec());
    let packet_len = value.read_var_i32()?;
    let packet_body_len = value.get_ref().len();
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
            let decompressed_bytes = Cursor::new(buf.to_vec());
            let mut d = ZlibDecoder::new(decompressed_bytes);
            d.write_all(value.get_ref())?;
            let finish = d.finish()?;
            Ok(finish)
        }
    }
}
