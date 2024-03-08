pub mod v1_20_4;

const LATEST_VERSION: &str = "1.20.4";
const LATEST_PROTOCOL_VERSION: i32 = 765;

pub(crate) use protocol_macro::packets;

mod protocol_macro {

    macro_rules! packets {
        ($protocol:ty, $(($connection_state:pat, $packet_id:expr, $typ:ty), )*) => {
            use crate::net::prelude::PacketReader;
            impl PacketReader for $protocol {
                fn read_packet(server: &mut Server, player: &mut Player) -> std::io::Result<()> {
                    let bytes = &mut player.packet_buf;
                    let packet_id = bytes.read_var_i32()?;
                    let connection_state = &player.session_relay.connection_state;
                    match (connection_state, packet_id) {
                        $(
                            ($connection_state, $packet_id) => {
                            println!("asdfqwer");
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
