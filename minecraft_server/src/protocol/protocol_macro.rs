macro_rules! protocols {
    ($server:ty, $player:ty, $latest_protocol:ty, $($protocol:ty, )*) => {

            impl crate::net::prelude::Server for $server {
                type Player = $player;

                fn read_packet(&mut self, player: &mut crate::net::prelude::Socket<$player>) -> std::io::Result<()> {
                    match player.session_relay.protocol_id {
                        0 => {
                        <$latest_protocol as crate::net::prelude::PacketReadHandler<$server>>::handle_packet_read(self, player)?;
                        },
                        <$latest_protocol as crate::net::prelude::ProtocolIdentifier>::PROTOCOL_ID => {
                            <$latest_protocol as crate::net::prelude::PacketReadHandler<$server>>::handle_packet_read(self, player)?;
                        }
                        $(
                        <$protocol as crate::net::prelude::ProtocolIdentifier>::PROTOCOL_ID => {
                            <$protocol as crate::net::prelude::PacketReadHandler<$server>>::handle_packet_read(self, player)?;
                        }
                        )*
                        n => {
                            use std::io::{Error, ErrorKind};
                            return Err(Error::new(
                                ErrorKind::InvalidInput,
                                format!("unknown protocol: {:?}", n),
                            ))
                        }
                    }
                    Ok(())
                }
            }
    };
}

macro_rules! packets {
        ($server:ty, $player:ty, $protocol:ty, $protocol_id:expr, $(($bound:pat, $connection_state:pat, $packet_id:expr, $typ:ty), )*) => {
            $(
                impl crate::net::prelude::PacketIdentifier<$player> for $typ {
                    fn get_protocol_id(&self, player: &mut crate::net::prelude::Socket<$player>) -> std::io::Result<i32> {
                        Ok($packet_id)
                    }
                }
            )*

            impl crate::net::prelude::ProtocolIdentifier for $protocol {
                const PROTOCOL_ID: i32 = $protocol_id;
            }

            impl crate::net::prelude::PacketReadHandler<$server> for $protocol {
                fn handle_packet_read(server: &mut $server, player: &mut crate::net::prelude::Socket<$player>) -> std::io::Result<()> {
                    let bytes = &mut player.packet_buf;
                    let packet_id = crate::io::prelude::VarIntRead::read_var_i32(bytes)?;
                    let connection_state = &player.session_relay.connection_state;
                    match (connection_state, packet_id) {
                        $(

                            ($connection_state, $packet_id) => {
                            <$typ as crate::net::prelude::PacketReadHandler<$server>>::handle_packet_read(server, player)?;
                        }
                        )*
                        _ => {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
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
pub(crate) use protocols;
