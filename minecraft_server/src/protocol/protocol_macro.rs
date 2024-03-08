pub enum Bound {
    Server,
    Client,
}

macro_rules! protocols {
    ($server:ty, $player:ty, $($protocol:ty)*) => {
        $(
            impl PacketReader<$player> for $protocol {
            }
        )*
    };
}

macro_rules! packets {
        ($server:ty, $player:ty, $protocol:ty, $(($bound:pat, $connection_state:pat, $packet_id:expr, $typ:ty), )*) => {
            use crate::net::prelude::{Socket, PacketIdentifier, PacketReader};
            use crate::server::prelude::Server;
            $(
                impl PacketIdentifier<$player> for $typ {
                    fn get_packet_id(&self, player: &mut Socket<$player>) -> std::io::Result<i32> {
                        Ok($packet_id)
                    }
                }
            )*

            impl PacketReader<$server, $player> for $protocol {
                fn read_packet(server: &mut Server, player: &mut Socket<$player>) -> std::io::Result<()> {
                    let bytes = &mut player.packet_buf;
                    let packet_id = bytes.read_var_i32()?;
                    let connection_state = &player.session_relay.connection_state;
                    match (connection_state, packet_id) {
                        $(

                            ($connection_state, $packet_id) => {
                            <$typ>::read_packet(server, player)?;
                        }
                        )*
                        _ => {
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                format!("{:?}[{:#04X?}] not exists", connection_state, packet_id),
                            ))
                        }
                    }
                    Ok(())
                }
            }
        };
    }

pub(crate) use packets;
