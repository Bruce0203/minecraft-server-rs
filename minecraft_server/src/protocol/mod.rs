pub mod v1_20_4;

const LATEST_VERSION: &str = "1.20.4";
const LATEST_PROTOCOL_VERSION: i32 = 765;

pub(crate) use protocol_macro::*;

mod protocol_macro {
    pub enum Bound {
        Server,
        Client,
    }

    macro_rules! packets {
        ($protocol:ty, $(($buond:pat, $connection_state:pat, $packet_id:expr, $typ:ty), )*) => {
            $(
                impl crate::net::prelude::PacketIdentifier for $typ {
                    fn get_packet_id(&self, player: &mut Player) -> std::io::Result<i32> {
                        Ok($packet_id)
                    }
                }
            )*

            use crate::net::prelude::PacketReader;
            impl PacketReader for $protocol {
                fn read_packet(server: &mut crate::server::prelude::Server, player: &mut Player) -> std::io::Result<()> {
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
}
