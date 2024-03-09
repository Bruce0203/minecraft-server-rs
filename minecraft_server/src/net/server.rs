use std::io::{Cursor, Error, ErrorKind, Read, Result, Write};

use flate2::write::ZlibDecoder;

use crate::io::prelude::{Decoder, VarIntRead};

use super::prelude::{PacketHandler, Socket};

pub trait Server: Sized {
    type Player: Default;

    const MAX_PACKET_BUFFER_SIZE: usize;

    fn read_packet(&mut self, player: &mut Socket<Self::Player>) -> Result<()>;

    fn handle_read_event(&mut self, player: &mut Socket<Self::Player>) -> Result<()> {
        Self::fill_read_buf_from_socket_stream(player)?;
        self.read_packet_from_read_buf(player)?;
        let write_buf = &player.write_buf.get_ref()[..player.write_buf.position() as usize];
        player.stream.write_all(write_buf)?;
        player.write_buf.set_position(0);
        Ok(())
    }

    fn fill_read_buf_from_socket_stream(player: &mut Socket<Self::Player>) -> Result<()> {
        let mut pos = player.read_buf.position() as usize;
        let read_len = player.stream.read(&mut player.read_buf.get_mut()[pos..])?;
        pos += read_len;
        if read_len == 0 || pos >= Self::MAX_PACKET_BUFFER_SIZE {
            Err(Error::new(ErrorKind::BrokenPipe, "BrokenPipe"))?
        }
        player.read_buf.set_position(pos as u64);
        Ok(())
    }

    fn process_packet_read<Packet: Decoder + PacketHandler<Self>>(
        &mut self,
        player: &mut Socket<Self::Player>,
    ) -> Result<()> {
        Packet::decode_from_read(&mut player.packet_buf)?.handle_packet(self, player)?;
        Ok(())
    }

    fn read_packet_from_read_buf(&mut self, player: &mut Socket<Self::Player>) -> Result<()> {
        let read_len = player.read_buf.position();
        player.read_buf.set_position(0);
        let mut do_read = || -> Result<()> {
            while player.read_buf.position() != read_len {
                let packet_len = player.read_buf.read_var_i32()?;
                let pos = player.read_buf.position() as usize;
                player.packet_buf = Cursor::new(Vec::from(
                    &player.read_buf.get_ref()[pos..pos + packet_len as usize],
                ));
                player.read_buf.read_exact(player.packet_buf.get_mut())?;
                player.process_decompression()?;
                self.read_packet(player)?;
            }
            player.read_buf.set_position(0);
            Ok(())
        };
        if let Err(err) = do_read() {
            player.read_buf.set_position(read_len);
            return Err(err);
        }
        Ok(())
    }
}
