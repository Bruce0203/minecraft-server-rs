use crate::{
    io::prelude::{Decoder, Encoder},
    protocol::macros::{packet_id, protocols, receiving_packets},
    server::prelude::{LoginPlayer, LoginServer},
};

use super::{
    configuration::{
        client_information::{
            ClientInformation, ClientInformationConfiguration, ClientInformationPlay,
        },
        feature_flags::FeatureFlags,
        finish_configuration::FinishConfiguration,
        plugin_message::{
            C2SPluginMessageConfiguration, C2SPluginMessagePlay, PluginMessage,
            S2CPluginMessageConfiguration, S2CPluginMessagePlay,
        },
        registry::RegistryData,
        server_data::ServerData,
    },
    login::{
        login_acknowledged::LoginAcknowledged, login_play::LoginPlay, login_start::LoginStart,
        login_success::LoginSuccess, set_compression::SetCompression,
    },
    play::{
        player_abilities::PlayerAbilities,
        player_info::PlayerInfoUpdate,
        set_center_chunk::SetCenterChunk,
        set_container_contents::SetContainerContent,
        set_container_property::SetContainerProperty,
        set_container_slot::SetContainerSlot,
        set_default_position::SetDefaultPosition,
        set_held_item::{C2SSetHeldItem, S2CSetHeldItem, SetHeldItem},
        set_render_distance::SetRenderDistance,
        set_simulation_distance::SetSimulationDistance,
        synchronize_player_position::SyncPlayerPosition,
        update_attributes::UpdateAttributes,
        update_time::UpdateTime, set_entity_metadata::SetEntityMetadata,
    },
    status::{PingRequest, PingResponse, StatusRequest, StatusResponse},
};
use crate::net::prelude::ConnectionState::*;

pub struct V1_20_4;

packet_id!(
    V1_20_4,
    (0x00, super::handshake::HandShake),
    (0x00, StatusResponse<'_>),
    (0x00, StatusRequest),
    (0x01, PingRequest),
    (0x01, PingResponse),
    (0x00, LoginStart),
    (0x03, SetCompression),
    (0x02, LoginSuccess),
    (0x03, LoginAcknowledged),
    (0x00, ClientInformationConfiguration),
    (0x09, ClientInformationPlay),
    (0x18, S2CPluginMessagePlay),
    (0x00, S2CPluginMessageConfiguration),
    (0x10, C2SPluginMessagePlay),
    (0x01, C2SPluginMessageConfiguration),
    (0x02, FinishConfiguration),
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
    (0x2C, C2SSetHeldItem),
    (0x51, S2CSetHeldItem),
    (0x36, PlayerAbilities),
    (0x05, RegistryData),
    (0x29, LoginPlay),
    (0x13, SetContainerContent),
    (0x14, SetContainerProperty),
    (0x15, SetContainerSlot),
    (0x56, SetEntityMetadata),
);

receiving_packets!(
    V1_20_4,
    (HandShake, super::handshake::HandShake),
    (Status, StatusRequest),
    (Status, PingRequest),
    (Login, LoginStart),
    (Login, LoginAcknowledged),
    (Confgiuration, ClientInformationConfiguration),
    (Play, ClientInformationPlay),
    (Play, C2SPluginMessagePlay),
    (Confgiuration, C2SPluginMessageConfiguration),
    (Confgiuration, FinishConfiguration),
    //(Play, C2SSetHeldItem),
);
