macro_rules! protocol {
    ($protocol:ty, $protocol_id:expr) => {
        impl crate::net::prelude::ProtocolId for $protocol {
            const PROTOCOL_ID: i32 = $protocol_id;
        }
    };
}

macro_rules! protocol_server {
    ($server:ty, $player:ty, $latest_protocol:ty, $($protocol:ty, )*) => {

        $(
            impl crate::net::prelude::Protocol for $protocol {
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
                        <$latest_protocol as crate::net::prelude::ProtocolId>::PROTOCOL_ID => {
                            <$latest_protocol as crate::net::prelude::PacketReadHandler<$server>>::handle_packet_read(self, player)?;
                        }
                        $(
                        <$protocol as crate::net::prelude::ProtocolId>::PROTOCOL_ID => {
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
    ($protocol:ty, $(($connection_state:pat, $typ:ty), )*) => {

        impl crate::net::prelude::PacketReadHandler<<$protocol as crate::net::prelude::Protocol>::Server> for $protocol {
            fn handle_packet_read(server: &mut <$protocol as crate::net::prelude::Protocol>::Server, player: &mut crate::net::prelude::Socket<<$protocol as crate::net::prelude::Protocol>::Player>) -> std::io::Result<()> {
                let bytes = &mut player.packet_buf;
                let packet_id = crate::io::prelude::VarIntRead::read_var_i32(bytes)?;
                let connection_state = &player.session_relay.connection_state;
                match (connection_state, packet_id) {
                    $(
                        ($connection_state, <$typ as crate::net::prelude::PacketId>::PACKET_ID) => {
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

macro_rules! packet_id {
    ($protocol:ty, $(($packet_id:expr, $typ:ty), )*) => {
        $(
            impl crate::net::prelude::PacketId for $typ {
                const PACKET_ID: i32 = $packet_id;
            }
        )*
    };
}

pub(crate) use packet_id;
pub(crate) use protocol;
pub(crate) use protocol_server;
pub(crate) use receiving_packets;
