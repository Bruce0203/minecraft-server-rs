macro_rules! protocols {
    ($server:ty, $player:ty, $latest_protocol:ty, $($protocol:ty, $protocol_id:expr, )*) => {

        $(
            impl crate::net::prelude::Protocol for $protocol {
                const PROTOCOL_ID: i32 = $protocol_id;
                type Player = $player;
                type Server = $server;
            }
            )*

            impl crate::net::prelude::Server for $server {
                type Player = $player;

                fn read_packet(&mut self, player: &mut crate::net::prelude::Socket<$player>) -> std::io::Result<()> {
                    match player.session_relay.protocol_id {
                        0 => {
                            <$latest_protocol as crate::net::prelude::PacketReadHandler<$server>>::handle_packet_read(self, player)?;
                        },
                        <$latest_protocol as crate::net::prelude::Protocol>::PROTOCOL_ID => {
                            <$latest_protocol as crate::net::prelude::PacketReadHandler<$server>>::handle_packet_read(self, player)?;
                        }
                        $(
                        <$protocol as crate::net::prelude::Protocol>::PROTOCOL_ID => {
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

macro_rules! receiving_packets {
    ($protocol:ty, $(($packet_id:expr, $connection_state:pat, $typ:ty), )*) => {
        $(
            impl crate::net::prelude::PacketId<<$protocol as crate::net::prelude::Protocol>::Player> for $typ {
                fn get_packet_id(&self, player: &mut crate::net::prelude::Socket<<$protocol as crate::net::prelude::Protocol>::Player>) -> std::io::Result<i32> {
                    Ok($packet_id)
                }
            }
        )*

        impl crate::net::prelude::PacketReadHandler<<$protocol as crate::net::prelude::Protocol>::Server> for $protocol {
            fn handle_packet_read(server: &mut <$protocol as crate::net::prelude::Protocol>::Server, player: &mut crate::net::prelude::Socket<<$protocol as crate::net::prelude::Protocol>::Player>) -> std::io::Result<()> {
                let bytes = &mut player.packet_buf;
                let packet_id = crate::io::prelude::VarIntRead::read_var_i32(bytes)?;
                let connection_state = &player.session_relay.connection_state;
                match (connection_state, packet_id) {
                    $(

                        ($connection_state, $packet_id) => {
                        <$typ as crate::net::prelude::PacketReadHandler<<$protocol as crate::net::prelude::Protocol>::Server>>::handle_packet_read(server, player)?;
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

macro_rules! sending_packets {
    ($protocol:ty, $(($packet_id:expr, $connection_state:pat, $typ:ty), )*) => {
        $(
            impl crate::net::prelude::PacketId<<$protocol as crate::net::prelude::Protocol>::Player> for $typ {
                fn get_packet_id(&self, player: &mut crate::net::prelude::Socket<<$protocol as crate::net::prelude::Protocol>::Player>) -> std::io::Result<i32> {
                    Ok($packet_id)
                }
            }
        )*
    };
}

pub(crate) use protocols;
pub(crate) use receiving_packets;
pub(crate) use sending_packets;
