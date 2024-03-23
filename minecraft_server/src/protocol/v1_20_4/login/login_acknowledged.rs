use std::io::{prelude::Read, Cursor, Error, Result, Write};

use crate::{
    io::prelude::{Buffer, Decoder, Encoder, U8Read, U8Write},
    net::prelude::{PacketHandler, PacketWriter, Socket},
    protocol::v1_20_4::configuration::{
        feature_flags::FeatureFlags,
        finish_configuration::FinishConfigurationS2c,
        registry::{
            Biome, ChatType, DamageType, Decoration, DimensionType, Effects, IntegerDistribution,
            MonsterSpawnLightLevel, Registry, RegistryData, RegistryEntry,
        },
    },
    server::prelude::{ConnectionState, GamePlayer, GameServer},
};

pub struct LoginAcknowledged {}

impl Encoder for LoginAcknowledged {
    fn encode_to_buffer(&self, buf: &mut Buffer) -> Result<()> {
        Ok(())
    }
}

impl Decoder for LoginAcknowledged {
    fn decode_from_read(reader: &mut Buffer) -> Result<Self> {
        Ok(LoginAcknowledged {})
    }
}

impl PacketHandler<GameServer> for LoginAcknowledged {
    fn handle_packet(
        &self,
        server: &mut GameServer,
        player: &mut Socket<GamePlayer>,
    ) -> Result<()> {
        player.session_relay.connection_state = ConnectionState::Confgiuration;

        let registry_data = RegistryData {
            chat_type_registry: Some(Registry {
                name: "minecraft:chat_type".to_string(),
                value: vec![RegistryEntry {
                    name: "minecraft:chat".to_string(),
                    id: 0,
                    element: ChatType {
                        chat: Decoration {
                            translation_key: "chat.type.text".to_string(),
                            style: None,
                            parameters: vec!["sender".to_string(), "content".to_string()],
                        },
                        narration: Decoration {
                            translation_key: "chat.type.text.narrate".to_string(),
                            style: None,
                            parameters: vec!["sender".to_string(), "content".to_string()],
                        },
                    },
                }],
            }),
            biome_registry: Some(Registry {
                name: "minecraft:worldgen/biome".to_string(),
                value: vec![RegistryEntry {
                    name: "minecraft:plains".to_string(),
                    id: 0,
                    element: Biome {
                        has_precipitation: false,
                        temperature: 0.5,
                        temperature_modifier: None,
                        downfall: 0.5,
                        effects: Effects {
                            fog_color: 12638463,
                            water_color: 4159204,
                            water_fog_color: 329011,
                            sky_color: 8103167,
                            foliage_color: None,
                            grass_color: None,
                            grass_color_modifier: None,
                            particle: None,
                            ambient_sound: None,
                            mood_sound: None,
                            additions_sound: None,
                            music: None,
                        },
                    },
                }],
            }),
            dimension_type_registry: Some(Registry {
                name: "minecraft:dimension_type".to_string(),
                value: vec![RegistryEntry {
                    name: "minecraft:overworld".to_string(),
                    id: 0,
                    element: DimensionType {
                        fixed_type: None,
                        has_skylight: true,
                        has_ceiling: false,
                        ultrawarm: false,
                        natural: true,
                        coordinate_scale: 1.0,
                        bed_works: true,
                        respawn_anchor_works: false,
                        min_y: -64,
                        height: 384,
                        logical_height: 384,
                        infiniburn: "#minecraft:infiniburn_overworld".to_string(),
                        effects: "minecraft:overworld".to_string(),
                        ambient_light: 0.0,
                        piglin_safe: false,
                        has_raids: true,
                        monster_spawn_light_level: MonsterSpawnLightLevel {
                            name: "minecraft:uniform".to_string(),
                            value: IntegerDistribution {
                                min_inclusive: 0,
                                max_inclusive: 7,
                            },
                        },
                        monster_spawn_block_light_limit: 0,
                    },
                }],
            }),
            trim_material_registry: Some(Registry {
                name: "minecraft:trim_material".to_string(),
                value: vec![],
            }),
            trim_pattern_registry: Some(Registry {
                name: "minecraft:trim_pattern".to_string(),
                value: vec![],
            }),
            damage_type_registry: Some(Registry {
                name: "minecraft:damage_type".to_string(),
                value: vec![
                    RegistryEntry {
                        name: "minecraft:arrow".to_string(),
                        id: 0,
                        element: DamageType {
                            message_id: "arrow".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:bad_respawn_point".to_string(),
                        id: 1,
                        element: DamageType {
                            message_id: "badRespawnPoint".to_string(),
                            scaling: "always".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: Some("intentional_game_design".to_string()),
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:cactus".to_string(),
                        id: 2,
                        element: DamageType {
                            message_id: "cactus".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:cramming".to_string(),
                        id: 3,
                        element: DamageType {
                            message_id: "cramming".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:dragon_breath".to_string(),
                        id: 4,
                        element: DamageType {
                            message_id: "dragonBreath".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:drown".to_string(),
                        id: 5,
                        element: DamageType {
                            message_id: "drown".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: Some("drowning".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:dry_out".to_string(),
                        id: 6,
                        element: DamageType {
                            message_id: "dryout".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:explosion".to_string(),
                        id: 7,
                        element: DamageType {
                            message_id: "explosion".to_string(),
                            scaling: "always".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:".to_string(),
                        id: 8,
                        element: DamageType {
                            message_id: "fall".to_string(),
                            scaling: "always".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: Some("fall_variants".to_string()),
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:anvil".to_string(),
                        id: 9,
                        element: DamageType {
                            message_id: "anvil".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:falling_block".to_string(),
                        id: 10,
                        element: DamageType {
                            message_id: "fallingBlock".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:falling_stalactite".to_string(),
                        id: 11,
                        element: DamageType {
                            message_id: "fallingStalactite".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:fireball".to_string(),
                        id: 12,
                        element: DamageType {
                            message_id: "fireball".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: Some("burning".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:fireworks".to_string(),
                        id: 13,
                        element: DamageType {
                            message_id: "fireworks".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:fly_into_wall".to_string(),
                        id: 14,
                        element: DamageType {
                            message_id: "flyIntoWall".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:freeze".to_string(),
                        id: 15,
                        element: DamageType {
                            message_id: "freeze".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: Some("freezing".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:generic".to_string(),
                        id: 16,
                        element: DamageType {
                            message_id: "generic".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:generic_kill".to_string(),
                        id: 17,
                        element: DamageType {
                            message_id: "genericKill".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:hot_floor".to_string(),
                        id: 18,
                        element: DamageType {
                            message_id: "hotFloor".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: Some("burning".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:in_fire".to_string(),
                        id: 19,
                        element: DamageType {
                            message_id: "inFire".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: Some("burning".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:in_wall".to_string(),
                        id: 20,
                        element: DamageType {
                            message_id: "inWall".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:indirect_magic".to_string(),
                        id: 21,
                        element: DamageType {
                            message_id: "indirectMagic".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:lava".to_string(),
                        id: 22,
                        element: DamageType {
                            message_id: "lava".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: Some("burning".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:lightning_bolt".to_string(),
                        id: 23,
                        element: DamageType {
                            message_id: "lightningBolt".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:magic".to_string(),
                        id: 24,
                        element: DamageType {
                            message_id: "magic".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:mob_attack".to_string(),
                        id: 25,
                        element: DamageType {
                            message_id: "mob".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:mob_attack_no_aggro".to_string(),
                        id: 26,
                        element: DamageType {
                            message_id: "mob".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:mob_projectile".to_string(),
                        id: 27,
                        element: DamageType {
                            message_id: "mob".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:on_fire".to_string(),
                        id: 28,
                        element: DamageType {
                            message_id: "onFire".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: Some("burning".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:out_of_world".to_string(),
                        id: 29,
                        element: DamageType {
                            message_id: "outOfWorld".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:outside_border".to_string(),
                        id: 30,
                        element: DamageType {
                            message_id: "outsideBorder".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:player_attack".to_string(),
                        id: 31,
                        element: DamageType {
                            message_id: "player".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:player_explosion".to_string(),
                        id: 32,
                        element: DamageType {
                            message_id: "explosion.player".to_string(),
                            scaling: "always".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:sonic_boom".to_string(),
                        id: 33,
                        element: DamageType {
                            message_id: "sonic_boom".to_string(),
                            scaling: "always".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:stalagmite".to_string(),
                        id: 34,
                        element: DamageType {
                            message_id: "stalagmite".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:starve".to_string(),
                        id: 35,
                        element: DamageType {
                            message_id: "starve".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:sting".to_string(),
                        id: 36,
                        element: DamageType {
                            message_id: "sting".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:sweet_berry_bush".to_string(),
                        id: 37,
                        element: DamageType {
                            message_id: "sweetBerryBush".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: Some("poking".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:thorns".to_string(),
                        id: 38,
                        element: DamageType {
                            message_id: "thorns".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: Some("thorns".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:thrown".to_string(),
                        id: 39,
                        element: DamageType {
                            message_id: "thrown".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:trident".to_string(),
                        id: 40,
                        element: DamageType {
                            message_id: "trident".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:unattributed_fireball".to_string(),
                        id: 41,
                        element: DamageType {
                            message_id: "onFire".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: Some("burning".to_string()),
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:wither".to_string(),
                        id: 42,
                        element: DamageType {
                            message_id: "wither".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.0,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                    RegistryEntry {
                        name: "minecraft:wither_skull".to_string(),
                        id: 43,
                        element: DamageType {
                            message_id: "witherSkull".to_string(),
                            scaling: "when_caused_by_living_non_player".to_string(),
                            exhaustion: 0.1,
                            effects: None,
                            death_message_type: None,
                        },
                    },
                ],
            }),
        };
        registry_data.send_packet(player)?;
        let mut buf = Buffer::new(vec![]);
        buf.write_u8(10);
        let mut inner_buf = Buffer::new(vec![]);
        nbt::to_writer(&mut inner_buf, &registry_data, None).unwrap();
        buf.write_all(&inner_buf.get_ref()[3..]);
        println!("{}", buf.position());
        buf.set_position(0);
        buf.read_u8();
        let value = nbt::Value::from_reader(10, &mut buf).unwrap();
        println!("read: {:#?}", value);
        let feature_flags = FeatureFlags {
            feature_flags: Vec::new(),
        };
        feature_flags.send_packet(player)?;
        let finish_configuration = FinishConfigurationS2c;
        finish_configuration.send_packet(player)?;
        Ok(())
    }
}
