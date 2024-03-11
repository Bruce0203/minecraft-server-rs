use std::io::{Cursor, Result, Write};

use serde::{Deserialize, Serialize};

use crate::{
    io::prelude::Encoder,
    net::prelude::{PacketId, Socket},
    server::{
        chat::ChatStyle,
        prelude::{GamePlayer, GameServer},
    },
};

#[derive(Serialize, Deserialize, Clone)]
pub struct RegistryData {
    #[serde(rename = "minecraft:chat_type")]
    pub chat_type_registry: Registry<ChatType>,
    #[serde(rename = "minecraft:worldgen/biome")]
    pub biome_registry: Registry<Biome>,
    #[serde(rename = "minecraft:dimension_type")]
    pub dimension_type_registry: Registry<DimensionType>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Registry<E> {
    #[serde(rename = "type")]
    pub name: String,
    pub value: Vec<RegistryEntry<E>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegistryEntry<V: Sized> {
    pub name: String,
    pub id: i32,
    pub element: V,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatType {
    pub chat: Decoration,
    pub narration: Decoration,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Decoration {
    pub translation_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<ChatStyle>,
    pub parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DamageType {
    pub message_id: String,
    pub scaling: String,
    pub exhausition: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub death_message_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct MonsterSpawnLightLevel {
    #[serde(rename = "type")]
    pub name: String,
    pub value: IntegerDistribution,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct IntegerDistribution {
    pub min_inclusive: i32,
    pub max_inclusive: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Biome {
    pub has_precipitation: bool,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: Effects,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Particle {
    pub options: ParticleOptions,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ParticleOptions {
    pub name: String,
    pub value: crate::server::prelude::particle::Particle,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AmbientSound {
    pub sound_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<f32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MoodSound {
    pub sound: String,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Music {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    pub replace_current_music: bool,
}

impl Encoder for RegistryData {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        let mut buffer = Cursor::new(Vec::new());
        nbt::to_writer(&mut buffer, self, Some(""))?;
        buf.write_all(&[10])?;
        buf.write_all(&buffer.get_ref()[3..])?;
        Ok(())
    }
}
