use std::{fs::File, io::{prelude::Read, Result, Write}};

use serde::{Deserialize, Serialize};

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, NbtNetworkRead, NbtNetworkWrite},
    net::prelude::{PacketId, Socket},
    server::{
        chat::ChatStyle,
        prelude::{GamePlayer, GameServer},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegistryData {
    //TODO wip
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Registry<E> {
    #[serde(rename = "type")]
    pub name: String,
    pub value: Vec<RegistryEntry<E>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegistryEntry<V: Sized> {
    pub name: String,
    pub id: i32,
    pub element: V,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrimPattern {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrimMaterial {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatType {
    pub chat: Decoration,
    pub narration: Decoration,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Decoration {
    pub translation_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ChatStyle>,
    pub parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DamageType {
    pub message_id: String,
    pub scaling: String,
    pub exhaustion: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_message_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DimensionType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_type: Option<i64>,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ultrawarm: bool,
    pub natural: bool,
    pub coordinate_scale: f64,
    pub bed_works: bool,
    pub respawn_anchor_works: bool,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub infiniburn: String,
    pub effects: String,
    pub ambient_light: f32,
    pub piglin_safe: bool,
    pub has_raids: bool,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub monster_spawn_block_light_limit: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MonsterSpawnLightLevel {
    #[serde(rename = "type")]
    pub name: String,
    pub value: IntegerDistribution,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntegerDistribution {
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Biome {
    pub has_precipitation: bool,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: Effects,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Effects {
    pub fog_color: i32,
    pub water_color: i32,
    pub water_fog_color: i32,
    pub sky_color: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliage_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color_modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particle: Option<Particle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ambient_sound: Option<AmbientSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood_sound: Option<MoodSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additions_sound: Option<AdditionsSound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<Music>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Particle {
    pub options: ParticleOptions,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParticleOptions {
    pub name: String,
    pub value: crate::server::prelude::particle::Particle,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AmbientSound {
    pub sound_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MoodSound {
    pub sound: String,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Music {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    pub replace_current_music: bool,
}

impl Encoder for RegistryData {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Decoder for RegistryData {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        let value = reader.read_nbt_compound()?;
        let mut buf = Buffer::new(vec![]);
        let mut file = File::create("registry_data.txt").unwrap();
        value.to_writer(&mut file)?;
        println!("{:?}", buf.get_ref());
        Ok(RegistryData {})
    }
}
