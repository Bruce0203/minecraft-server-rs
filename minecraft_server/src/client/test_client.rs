use std::{
    backtrace::Backtrace,
    env,
    io::{Cursor, Result, Write},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
    vec,
};

use mio::{net::TcpStream, Token};
use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;

use crate::{
    net::{
        prelude::{PacketHandler, PacketWriter, ProtocolId},
        socket::Socket,
    },
    protocol::{
        macros::{protocol, protocol_server, receiving_packets},
        v1_20_4::{
            configuration::{
                client_information::{
                    ChatMode, ClientInformation, ClientInformationConf, DisplaySkinParts,
                },
                feature_flags::FeatureFlags,
                finish_configuration::{FinishConfigurationC2s, FinishConfigurationS2c},
                plugin_message::{
                    PluginMessage, PluginMessageConfC2s, PluginMessageConfS2c, PluginMessagePlayS2c,
                },
                registry::RegistryData,
                update_tags::UpdateTags,
            },
            handshake::{HandShake, NextState},
            login::{
                login_acknowledged::LoginAcknowledged, login_start::LoginStart,
                login_success::LoginSuccess, set_compression::SetCompression,
            },
            play::keep_alive::{KeepAlive, KeepAliveConfC2s, KeepAliveConfS2c},
            v1_20_4::MinecraftServerV1_20_4,
        },
    },
    server::prelude::{ConnectionState, GamePlayer, GameServer, MainHand},
};

const MAX_PACKET_BUFFER_SIZE: usize = 100_000;

pub struct MinecraftClientV1_20_4;
protocol!(MinecraftClientV1_20_4, MinecraftServerV1_20_4::PROTOCOL_ID);

receiving_packets!(
    MinecraftClientV1_20_4,
    (ConnectionState::Login, LoginSuccess),
    (ConnectionState::Login, SetCompression),
    (ConnectionState::Confgiuration, PluginMessageConfS2c),
    (ConnectionState::Play, PluginMessagePlayS2c),
    (ConnectionState::Confgiuration, FeatureFlags),
    (ConnectionState::Confgiuration, KeepAliveConfS2c),
    (ConnectionState::Confgiuration, RegistryData),
    (ConnectionState::Confgiuration, UpdateTags),
    (ConnectionState::Confgiuration, FinishConfigurationS2c),
);

#[derive(Default)]
pub struct Client {}
pub struct ClientPool {}

protocol_server!(
    ClientPool,
    Client,
    MinecraftClientV1_20_4,
    MinecraftClientV1_20_4,
);

#[ignore]
#[test]
fn test_client() {
    let addr = env::var("MY_IP").unwrap().parse().unwrap();
    let mut stream = std::net::TcpStream::connect(addr).unwrap();
    let server = &mut ClientPool {};
    let mut player = &mut Socket::new::<MAX_PACKET_BUFFER_SIZE>(
        TcpStream::from_std(stream),
        Token(1),
        addr,
        Client::default(),
    );
    HandShake {
        protocol_version: MinecraftClientV1_20_4::PROTOCOL_ID,
        server_address: addr.to_string(),
        server_port: addr.port(),
        next_state: NextState::Login,
    }
    .send_packet(player)
    .unwrap();
    let random_hash_of_player_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    player.session_relay.connection_state = ConnectionState::Login;
    LoginStart {
        name: format!("Test_{}", random_hash_of_player_name),
        player_uuid: Uuid::new_v4(),
    }
    .send_packet(player);
    send_written_packets(player);
    read_packets(player, server);
    LoginAcknowledged {}.send_packet(player).unwrap();
    PluginMessageConfC2s(PluginMessage {
        channel: "minecraft:brand".to_string(),
        data: "vanilla".to_string().into(),
    })
    .send_packet(player)
    .unwrap();
    send_written_packets(player);
    ClientInformationConf(ClientInformation {
        locale: "ko_KR".to_string().into(),
        view_distance: 2,
        chat_mode: ChatMode::Enabled,
        chat_colors: true,
        display_skin_parts: DisplaySkinParts::None,
        main_hand: MainHand::Right,
        enable_text_filtering: true,
        allow_server_listings: true,
    });
    send_written_packets(player);
    read_packets(player, server);
    //KeepAliveConfC2s(KeepAlive {
    //    id: SystemTime::now()
    //        .duration_since(UNIX_EPOCH)
    //        .unwrap()
    //        .as_millis() as i64,
    //})
    //.send_packet(player)
    //.unwrap();
    // send_written_packets(player);
    // println!("bruh");
    // read_packets(player, server);
}

impl PacketHandler<ClientPool> for FeatureFlags {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

impl PacketHandler<ClientPool> for PluginMessageConfS2c {
    fn handle_packet(
        &self,
        server: &mut ClientPool,
        player: &mut Socket<<ClientPool as crate::net::prelude::Server>::Player>,
    ) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

impl PacketHandler<ClientPool> for PluginMessagePlayS2c {
    fn handle_packet(
        &self,
        server: &mut ClientPool,
        player: &mut Socket<<ClientPool as crate::net::prelude::Server>::Player>,
    ) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

impl PacketHandler<ClientPool> for LoginSuccess {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        println!("{:?}", self);
        player.session_relay.connection_state = ConnectionState::Confgiuration;
        Ok(())
    }
}

impl PacketHandler<ClientPool> for SetCompression {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        player.session_relay.compression_threshold = self.compression_threshold;
        println!("{:?}", self);
        Ok(())
    }
}

impl PacketHandler<ClientPool> for KeepAliveConfS2c {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

impl PacketHandler<ClientPool> for RegistryData {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        println!("RegistryData!!");
        Ok(())
    }
}

fn send_written_packets(player: &mut Socket<Client>) {
    player
        .stream
        .write_all(&player.write_buf.get_ref()[..player.write_buf.position() as usize])
        .unwrap();
    let write_buf = &player.write_buf.get_ref()[..player.write_buf.position() as usize];
    println!("{:?}", write_buf);
    player.write_buf = Cursor::new(vec![]);
}

fn read_packets(player: &mut Socket<Client>, server: &mut ClientPool) {
    player
        .handle_read_event::<MAX_PACKET_BUFFER_SIZE, _>(server)
        .unwrap();
}

impl PacketHandler<ClientPool> for UpdateTags {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        println!("UpdateTags!");
        Ok(())
    }
}

impl PacketHandler<ClientPool> for FinishConfigurationS2c {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        println!("FinishConfiguration");
        FinishConfigurationC2s.send_packet(player)?;
        player.session_relay.connection_state = ConnectionState::Play;
        Ok(())
    }
}
