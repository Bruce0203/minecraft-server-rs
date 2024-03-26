use std::{
    backtrace::Backtrace,
    env,
    io::{Cursor, Result, Write},
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
    vec,
};

use mio::{net::TcpStream, Interest, Registry, Token};
use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;

use crate::{
    io::prelude::{Buffer, Encoder},
    net::{
        prelude::{
            MaxPacketBufferSize, PacketHandler, PacketWriter, ProtocolId, Selector, SelectorTicker,
            SelectorUpdateListener, Server, SocketSelector,
        },
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
                login_acknowledged::LoginAcknowledged, login_play::LoginPlay,
                login_start::LoginStart, login_success::LoginSuccess,
                set_compression::SetCompression,
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
    (ConnectionState::Play, LoginPlay),
);

#[derive(Default)]
pub struct Client {}

pub struct ClientPool {
    pub start_time: SystemTime,
    pub last_tick_time: SystemTime,
}

impl MaxPacketBufferSize for ClientPool {
    const MAX_PACKET_BUFFER_SIZE: usize = 100_000;
}

protocol_server!(
    ClientPool,
    Client,
    MinecraftClientV1_20_4,
    MinecraftClientV1_20_4,
);

#[ignore]
#[test]
fn test_client() {
    let server = ClientPool {
        start_time: SystemTime::now(),
        last_tick_time: SystemTime::UNIX_EPOCH,
    };
    let mut selector = SocketSelector::new(server);
    selector.run();
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
        player: &mut Socket<<ClientPool as Server>::Player>,
    ) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

impl PacketHandler<ClientPool> for PluginMessagePlayS2c {
    fn handle_packet(
        &self,
        server: &mut ClientPool,
        player: &mut Socket<<ClientPool as Server>::Player>,
    ) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

impl PacketHandler<ClientPool> for LoginSuccess {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        player.session_relay.connection_state = ConnectionState::Confgiuration;
        LoginAcknowledged {}.send_packet(player)?;
        PluginMessageConfC2s(PluginMessage {
            channel: "minecraft:brand".to_string(),
            data: "vanilla".to_string().into(),
        })
        .send_packet(player)?;
        ClientInformationConf(ClientInformation {
            locale: "ko_KR".to_string().into(),
            view_distance: 2,
            chat_mode: ChatMode::Enabled,
            chat_colors: true,
            display_skin_parts: DisplaySkinParts::None,
            main_hand: MainHand::Right,
            enable_text_filtering: true,
            allow_server_listings: true,
        })
        .send_packet(player)?;
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
        //println!("{:?}", self);
        println!("RegistryData!!");
        Ok(())
    }
}

fn send_written_packets(player: &mut Socket<Client>) {
    player
        .stream
        .write_all(&player.write_buf.get_ref()[..player.write_buf.position() as usize])
        .unwrap();
    player.write_buf = Cursor::new(vec![]);
}

fn read_packets(player: &mut Socket<Client>, server: &mut ClientPool) {
    player.handle_read_event(server).unwrap();
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

impl PacketHandler<ClientPool> for LoginPlay {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        println!("LoginPlay!!!: {:?}", self);
        Ok(())
    }
}

impl SelectorUpdateListener<ClientPool> for ClientPool {
    fn on_update(selector: &mut SocketSelector<ClientPool>) {
        let tick = Duration::from_millis(50);
        let now = SystemTime::now();
        if now.duration_since(selector.server.last_tick_time).unwrap() >= tick {
            selector.on_tick();
            selector.server.last_tick_time = now;
        }
    }

    fn on_init(selector: &mut SocketSelector<ClientPool>) {
        let addr = env::var("MY_IP").unwrap().parse().unwrap();
        selector
            .connect_client(addr, |player| {
                HandShake {
                    protocol_version: MinecraftClientV1_20_4::PROTOCOL_ID,
                    server_address: addr.to_string(),
                    server_port: addr.port(),
                    next_state: NextState::Login,
                }
                .send_packet(player)?;
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
                .send_packet(player)?;
                send_written_packets(player);
                Ok(())
            })
            .unwrap();
    }
}

impl SelectorTicker for SocketSelector<ClientPool> {
    fn on_tick(&mut self) {
        println!("tick");
    }
}
