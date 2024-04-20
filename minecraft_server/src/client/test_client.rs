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
                finish_configuration::{
                    FinishConfigurationAcknowledgedC2s, FinishConfigurationS2c,
                },
                plugin_message::{
                    PluginMessage, PluginMessageConfC2s, PluginMessageConfS2c, PluginMessagePlayS2c,
                },
                registry::RegistryData,
                server_data::ServerData,
                update_tags::UpdateTags,
            },
            handshake::{HandShake, NextState},
            login::{
                login_acknowledged::LoginAcknowledged, login_play::LoginPlay,
                login_start::LoginStart, login_success::LoginSuccess,
                set_compression::SetCompression,
            },
            play::{
                block_update::BlockUpdate,
                bundle_delimiter::BundleDelimiter,
                change_difficulty::ChangeDifficultyS2c,
                combat_death::CombatDeath,
                commands::Commands,
                confirm_teleportation::ConfirmTeleportation,
                damage_event::DamageEvent,
                end_combat::EndCombat,
                enter_combat::EnterCombat,
                entity_event::EntityEvent,
                game_event::GameEvent,
                hurt_animation::HurtAnimation,
                initialize_world_border::InitializeWorldBorder,
                keep_alive::{
                    KeepAlive, KeepAliveConfC2s, KeepAliveConfS2c, KeepAlivePlayC2s,
                    KeepAlivePlayS2c,
                },
                player_abilities::PlayerAbilities,
                player_info::PlayerInfoUpdate,
                pong::PongC2s,
                remove_entities::RemoveEntities,
                set_block_destroy_stage::SetBlockDestoryStage,
                set_center_chunk::SetCenterChunk,
                set_container_contents::SetContainerContent,
                set_container_slot::SetContainerSlot,
                set_default_position::SetDefaultPosition,
                set_entity_metadata::SetEntityMetadata,
                set_entity_velocity::SetEntityVelocity,
                set_expereience::SetExperience,
                set_head_rotation::SetHeadRotation,
                set_health::SetHealth,
                set_held_item::SetHeldItemS2c,
                set_render_distance::SetRenderDistance,
                set_simulation_distance::SetSimulationDistance,
                set_ticking_state::SetTickingState,
                sound_effect::SoundEffect,
                spawn_entity::SpawnEntity,
                step_tick::StepTick,
                synchronize_player_position::SyncPlayerPosition,
                system_chat_message::SystemChatMessage,
                teleport_entity::TeleportEntity,
                update_advancements::UpdateAdvancements,
                update_attributes::UpdateAttributes,
                update_entity_position::UpdateEntityPosition,
                update_entity_position_and_rotation::UpdateEntityPositionAndRotation,
                update_entity_rotation::UpdateEntityRotation,
                update_light::UpdateLight,
                update_receipe_book::UpdateReceipeBook,
                update_receipes::UpdateReceipes,
                update_time::UpdateTime,
            },
            v1_20_4::MinecraftServerV1_20_4,
        },
    },
    server::{
        chunk::Chunk,
        prelude::{ConnectionState, GamePlayer, GameServer, MainHand},
    },
};

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
    (ConnectionState::Play, ChangeDifficultyS2c),
    (ConnectionState::Play, PlayerAbilities),
    (ConnectionState::Play, SetHeldItemS2c),
    (ConnectionState::Play, UpdateReceipes),
    (ConnectionState::Play, EntityEvent),
    (ConnectionState::Play, UpdateReceipeBook),
    (ConnectionState::Play, SyncPlayerPosition),
    (ConnectionState::Play, ServerData),
    (ConnectionState::Play, SystemChatMessage),
    (ConnectionState::Play, PlayerInfoUpdate),
    (ConnectionState::Play, SetRenderDistance),
    (ConnectionState::Play, SetSimulationDistance),
    (ConnectionState::Play, SetCenterChunk),
    (ConnectionState::Play, InitializeWorldBorder),
    (ConnectionState::Play, UpdateTime),
    (ConnectionState::Play, SetDefaultPosition),
    (ConnectionState::Play, GameEvent),
    (ConnectionState::Play, SetTickingState),
    (ConnectionState::Play, StepTick),
    (ConnectionState::Play, SetContainerContent),
    (ConnectionState::Play, SetContainerSlot),
    (ConnectionState::Play, SetEntityMetadata),
    (ConnectionState::Play, Commands),
    (ConnectionState::Play, UpdateAttributes),
    (ConnectionState::Play, UpdateAdvancements),
    (ConnectionState::Play, SetHealth),
    (ConnectionState::Play, SetExperience),
    (ConnectionState::Play, SoundEffect),
    (ConnectionState::Play, Chunk),
    (ConnectionState::Play, BundleDelimiter),
    (ConnectionState::Play, SpawnEntity),
    (ConnectionState::Play, UpdateEntityPosition),
    (ConnectionState::Play, SetEntityVelocity),
    (ConnectionState::Play, UpdateEntityPositionAndRotation),
    (ConnectionState::Play, UpdateEntityRotation),
    (ConnectionState::Play, SetHeadRotation),
    (ConnectionState::Play, TeleportEntity),
    (ConnectionState::Play, RemoveEntities),
    (ConnectionState::Play, EnterCombat),
    (ConnectionState::Play, DamageEvent),
    (ConnectionState::Play, KeepAlivePlayS2c),
    (ConnectionState::Play, HurtAnimation),
    (ConnectionState::Play, BlockUpdate),
    (ConnectionState::Play, CombatDeath),
    (ConnectionState::Play, EndCombat),
    (ConnectionState::Play, SetBlockDestoryStage),
    (ConnectionState::Play, UpdateLight),
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
        Ok(())
    }
}

impl PacketHandler<ClientPool> for KeepAlivePlayS2c {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        KeepAlivePlayC2s(self.0).send_packet(player)?;
        Ok(())
    }
}

impl PacketHandler<ClientPool> for RegistryData {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        println!("{:?}", self);
        Ok(())
    }
}

impl PacketHandler<ClientPool> for KeepAliveConfS2c {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        player.send_packet(&KeepAlivePlayC2s(self.0))?;
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

impl PacketHandler<ClientPool> for FinishConfigurationS2c {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        FinishConfigurationAcknowledgedC2s.send_packet(player)?;
        player.session_relay.connection_state = ConnectionState::Play;
        Ok(())
    }
}

impl PacketHandler<ClientPool> for SyncPlayerPosition {
    fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
        ConfirmTeleportation {
            teleport_id: self.teleport_id,
        }
        .send_packet(player)?;
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
        //tick
    }
}

macro_rules! empty_packet_handler {
    ($($typ:ty, )*) => {
        $(
            impl PacketHandler<ClientPool> for $typ {
                fn handle_packet(&self, server: &mut ClientPool, player: &mut Socket<Client>) -> Result<()> {
                    Ok(())
                }
            }
        )*
    };
}

empty_packet_handler!(
    LoginPlay,
    FeatureFlags,
    PluginMessageConfS2c,
    PluginMessagePlayS2c,
    PlayerAbilities,
    ChangeDifficultyS2c,
    SetHeldItemS2c,
    UpdateReceipes,
    EntityEvent,
    ServerData,
    UpdateReceipeBook,
    UpdateTime,
    InitializeWorldBorder,
    SetCenterChunk,
    SetRenderDistance,
    SetSimulationDistance,
    PlayerInfoUpdate,
    SystemChatMessage,
    SetDefaultPosition,
    GameEvent,
    SetTickingState,
    StepTick,
    SetContainerContent,
    SetContainerSlot,
    SetEntityMetadata,
    Commands,
    UpdateAttributes,
    UpdateAdvancements,
    SetHealth,
    SetExperience,
    SoundEffect,
    Chunk,
    BundleDelimiter,
    SpawnEntity,
    UpdateTags,
    UpdateEntityPosition,
    SetEntityVelocity,
    UpdateEntityPositionAndRotation,
    UpdateEntityRotation,
    SetHeadRotation,
    TeleportEntity,
    RemoveEntities,
    EnterCombat,
    DamageEvent,
    HurtAnimation,
    BlockUpdate,
    CombatDeath,
    EndCombat,
    SetBlockDestoryStage,
    UpdateLight,
);
