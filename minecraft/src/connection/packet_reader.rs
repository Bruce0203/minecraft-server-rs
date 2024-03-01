use std::io::{Error, ErrorKind, Write};

use bytes::{Buf, BufMut, Bytes, BytesMut};
use common_server::var_int::VarIntRead;
use flate2::write::ZlibDecoder;

use super::SessionRelay;

pub(super) fn read_packet_id_and_payload(
    buf: &[u8],
    session_relay: &mut SessionRelay,
) -> std::io::Result<BytesMut> {
    let value = BytesMut::from(buf);
    let mut reader = value.reader();
    let packet_len = reader.read_var_i32()?;
    let value = reader.into_inner();
    let packet_body_len = value.len();
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
        let mut reader = value.reader();
        let decompressed_len = reader.read_var_i32()?;
        if compression_threshold as usize > packet_body_len {
            Ok(reader.into_inner())
        } else {
            println!("start decompressing");
            println!("{}", decompressed_len);
            let decompressed_bytes = BytesMut::new();
            let decompressed_bytes_writer = decompressed_bytes.writer();
            let mut d = ZlibDecoder::new(decompressed_bytes_writer);
            d.write_all(&reader.into_inner())?;
            let finish = d.finish()?.into_inner();
            Ok(finish)
        }
    }
}
