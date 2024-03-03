use std::{
    io::{Cursor, Error, ErrorKind, Read, Write},
    ops::Deref,
};

use flate2::write::ZlibDecoder;
use mc_io::var_int::{self, VarIntRead};

use super::prelude::SessionRelay;

pub(super) fn read_packet_id_and_payload(
    value: &[u8],
    session_relay: &mut SessionRelay,
) -> std::io::Result<Cursor<Vec<u8>>> {
    println!("readstart");
    let compression_threshold = session_relay.compression_threshold;
    if compression_threshold == -1 {
        println!("readend");
        return Ok(Cursor::new(value.to_vec()));
    } else {
        let (decompressed_len, read_len) = var_int::read_var_i32_fast(value)?;
        println!("{:?}", value);
        if compression_threshold > decompressed_len {
            Ok(Cursor::new(value.to_vec()))
        } else {
            let mut d = ZlibDecoder::new(Cursor::new(Vec::new()));
            println!("{:?}", &value[read_len..]);
            d.write_all(&value[read_len..])?;
            let finish = d.finish()?;
            println!("readend");
            Ok(finish)
        }
    }
}
