use crate::{
    io::prelude::{Decoder, Encoder},
    net::prelude::PacketHandler,
    protocol::{
        macros::{packet_id, protocol, protocol_server, receiving_packets},
        v1_20_4::configuration::acknowledge_finish_configuration::AcknowledgeFinishConfiguration,
    },
    server::{
        chunk::Chunk,
        prelude::{GamePlayer, GameServer},
    },
};

use super::{
    configuration::{
        client_information::{ClientInformation, ClientInformationConf, ClientInformationPlay},
        feature_flags::FeatureFlags,
        finish_configuration::{FinishConfigurationC2s, FinishConfigurationS2c},
        plugin_message::{
            PluginMessage, PluginMessageConfC2s, PluginMessageConfS2c, PluginMessagePlayC2s,
            PluginMessagePlayS2c,
        },
        registry::RegistryData,
        server_data::ServerData,
        update_tags::UpdateTags,
    },
    login::{
        login_acknowledged::LoginAcknowledged, login_play::LoginPlay, login_start::LoginStart,
        login_success::LoginSuccess, set_compression::SetCompression,
    },
    play::{
        change_difficulty::ChangeDifficultyS2c,
        commands::Commands,
        entity_event::EntityEvent,
        game_event::GameEvent,
        initialize_world_border::InitializeWorldBorder,
        keep_alive::{KeepAliveConfC2s, KeepAliveConfS2c, KeepAlivePlayC2s, KeepAlivePlayS2c},
        player_abilities::PlayerAbilities,
        player_info::PlayerInfoUpdate,
        set_center_chunk::SetCenterChunk,
        set_container_contents::SetContainerContent,
        set_container_property::SetContainerProperty,
        set_container_slot::SetContainerSlot,
        set_default_position::SetDefaultPosition,
        set_entity_metadata::SetEntityMetadata,
        set_expereience::SetExperience,
        set_health::SetHealth,
        set_held_item::{SetHeldItem, SetHeldItemC2s, SetHeldItemS2c},
        set_render_distance::SetRenderDistance,
        set_simulation_distance::SetSimulationDistance,
        set_ticking_state::SetTickingState,
        sound_effect::SoundEffect,
        step_tick::StepTick,
        synchronize_player_position::SyncPlayerPosition,
        system_chat_message::SystemChatMessage,
        update_advancements::UpdateAdvancements,
        update_attributes::UpdateAttributes,
        update_light::UpdateLight,
        update_receipe_book::UpdateReceipeBook,
        update_receipes::UpdateReceipes,
        update_time::UpdateTime,
    },
    status::{PingRequest, PingResponse, StatusRequest, StatusResponse},
};
use crate::server::prelude::ConnectionState::*;

pub struct MinecraftServerV1_20_4;
protocol!(MinecraftServerV1_20_4, 765);

packet_id!(
    (0x00, super::handshake::HandShake),
    (0x00, StatusResponse<'_>),
    (0x00, StatusRequest),
    (0x01, PingRequest),
    (0x01, PingResponse),
    (0x00, LoginStart),
    (0x03, SetCompression),
    (0x02, LoginSuccess),
    (0x03, LoginAcknowledged),
    (0x00, ClientInformationConf),
    (0x09, ClientInformationPlay),
    (0x09, UpdateTags),
    (0x18, PluginMessagePlayS2c),
    (0x00, PluginMessageConfS2c),
    (0x10, PluginMessagePlayC2s),
    (0x01, PluginMessageConfC2s),
    (0x02, FinishConfigurationS2c),
    (0x02, FinishConfigurationC2s),
    (0x08, FeatureFlags),
    (0x3E, SyncPlayerPosition),
    (0x62, UpdateTime),
    (0x71, UpdateAttributes),
    (0x52, SetCenterChunk),
    (0x60, SetSimulationDistance),
    (0x53, SetRenderDistance),
    (0x3C, PlayerInfoUpdate),
    (0x49, ServerData),
    (0x54, SetDefaultPosition),
    (0x2C, SetHeldItemC2s),
    (0x51, SetHeldItemS2c),
    (0x36, PlayerAbilities),
    (0x05, RegistryData),
    (0x29, LoginPlay),
    (0x13, SetContainerContent),
    (0x14, SetContainerProperty),
    (0x15, SetContainerSlot),
    (0x56, SetEntityMetadata),
    (0x5B, SetHealth),
    (0x0B, ChangeDifficultyS2c),
    (0x25, Chunk),
    (0x69, SystemChatMessage),
    (0x15, KeepAlivePlayC2s),
    (0x03, KeepAliveConfS2c),
    (0x24, KeepAlivePlayS2c),
    (0x03, KeepAliveConfC2s),
    (0x28, UpdateLight),
    (0x02, AcknowledgeFinishConfiguration),
    (0x73, UpdateReceipes),
    (0x1D, EntityEvent),
    (0x3F, UpdateReceipeBook),
    (0x23, InitializeWorldBorder),
    (0x20, GameEvent),
    (0x6E, SetTickingState),
    (0x6F, StepTick),
    (0x11, Commands),
    (0x70, UpdateAdvancements),
    (0x5A, SetExperience),
    (0x66, SoundEffect),
);

receiving_packets!(
    MinecraftServerV1_20_4,
    (HandShake, super::handshake::HandShake),
    (Status, StatusRequest),
    (Status, PingRequest),
    (Login, LoginStart),
    (Login, LoginAcknowledged),
    (Confgiuration, ClientInformationConf),
    (Play, ClientInformationPlay),
    (Play, PluginMessagePlayC2s),
    (Confgiuration, PluginMessageConfC2s),
    (Confgiuration, FinishConfigurationC2s),
    //(Play, C2SSetHeldItem),
    (Play, KeepAlivePlayC2s),
    (Confgiuration, AcknowledgeFinishConfiguration),
);
