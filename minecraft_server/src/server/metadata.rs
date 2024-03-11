use std::{
    io::{prelude::Write, Result},
    panic::Location,
};

use bitflags::bitflags;
use delegate::delegate;

use crate::{
    io::prelude::{Decoder, Encoder, OptionWrite, U8Write, VarInt, VarIntWrite, WriteBool},
    server::{
        coordinates::Position,
        prelude::{Chat, MainHand},
    },
};

use super::prelude::Pose;

pub trait Metadata {
    fn get_metadata_id(&self) -> i32;
}

pub struct EntityMetadata {
    index0: EntityMetadataIndex0,
    air_ticks: VarInt,
    custom_name: Option<Box<Chat>>,
    is_custom_name_visible: bool,
    is_silent: bool,
    has_no_gravity: bool,
    pose: Pose,
    tick_frozen_is_powdered_snow: VarInt,
}

bitflags! {
    pub struct EntityMetadataIndex0 : u8 {
        const IsOnFire = 0x01;
        const IsCrouching = 0x02;

        #[deprecated(since="0.1.0", note="unused")]
        const IsRiding = 0x04;

        const IsSprinting = 0x08;
        const IsSwimming = 0x10;
        const IsInvisible = 0x20;
        const HasGlowingEffect = 0x40;
        const IsFlyingWithAnElytra = 0x80;

        const None = 0x00;
    }
}

impl Default for EntityMetadataIndex0 {
    fn default() -> Self {
        EntityMetadataIndex0::None
    }
}

impl Default for EntityMetadata {
    fn default() -> Self {
        Self {
            index0: Default::default(),
            air_ticks: 0.into(),
            custom_name: None,
            is_custom_name_visible: false,
            is_silent: false,
            has_no_gravity: false,
            pose: Pose::default(),
            tick_frozen_is_powdered_snow: 0.into(),
        }
    }
}

pub trait EntityMeta {
    fn has_glowing_effect(&self) -> bool;
    fn set_glowing_effect(&mut self, value: bool);

    fn is_crouching(&self) -> bool;
    fn set_crouching(&mut self, value: bool);

    fn is_flying_with_an_eytra(&self) -> bool;
    fn set_flying_with_an_elytra(&mut self, value: bool);

    fn is_invisible(&self) -> bool;
    fn set_invisible(&mut self, value: bool);

    fn is_on_fire(&self) -> bool;
    fn set_on_fire(&mut self, value: bool);

    fn is_sprinting(&self) -> bool;
    fn set_sprinting(&mut self, value: bool);

    fn is_swimming(&self) -> bool;
    fn set_swimming(&mut self, value: bool);

    fn air_ticks(&self) -> i32;
    fn set_air_ticks(&mut self, value: i32);

    fn custom_name(&self) -> &Option<Box<Chat>>;
    fn set_custom_name(&mut self, value: Option<Box<Chat>>);

    fn is_custom_name_visible(&self) -> bool;
    fn set_custom_name_visible(&mut self, value: bool);

    fn is_silent(&self) -> bool;
    fn set_silent(&mut self, value: bool);

    fn has_no_gravity(&self) -> bool;
    fn set_no_gravity(&mut self, value: bool);

    fn pose(&self) -> Pose;
    fn set_pose(&mut self, value: Pose);

    fn tick_frozen_is_powdered_snow(&self) -> i32;
    fn set_tick_frozen_is_powdered_snow(&mut self, value: i32);
}

impl EntityMeta for EntityMetadata {
    fn has_glowing_effect(&self) -> bool {
        self.index0
            .intersects(EntityMetadataIndex0::HasGlowingEffect)
    }

    fn is_crouching(&self) -> bool {
        self.index0.intersects(EntityMetadataIndex0::IsCrouching)
    }

    fn is_flying_with_an_eytra(&self) -> bool {
        self.index0
            .intersects(EntityMetadataIndex0::IsFlyingWithAnElytra)
    }

    fn is_invisible(&self) -> bool {
        self.index0.intersects(EntityMetadataIndex0::IsInvisible)
    }

    fn is_on_fire(&self) -> bool {
        self.index0.intersects(EntityMetadataIndex0::IsOnFire)
    }

    fn is_sprinting(&self) -> bool {
        self.index0.intersects(EntityMetadataIndex0::IsSprinting)
    }

    fn is_swimming(&self) -> bool {
        self.index0.intersects(EntityMetadataIndex0::IsSwimming)
    }

    fn air_ticks(&self) -> i32 {
        self.air_ticks.into()
    }

    fn custom_name(&self) -> &Option<Box<Chat>> {
        &Box::new(self.custom_name)
    }

    fn is_custom_name_visible(&self) -> bool {
        self.is_custom_name_visible
    }

    fn is_silent(&self) -> bool {
        self.is_silent
    }

    fn has_no_gravity(&self) -> bool {
        self.has_no_gravity
    }

    fn pose(&self) -> Pose {
        self.pose
    }

    fn tick_frozen_is_powdered_snow(&self) -> i32 {
        self.tick_frozen_is_powdered_snow.into()
    }

    fn set_glowing_effect(&mut self, value: bool) {
        self.index0 |= EntityMetadataIndex0::HasGlowingEffect
    }

    fn set_crouching(&mut self, value: bool) {
        self.index0 |= EntityMetadataIndex0::IsCrouching
    }

    fn set_flying_with_an_elytra(&mut self, value: bool) {
        self.index0 |= EntityMetadataIndex0::IsFlyingWithAnElytra
    }

    fn set_invisible(&mut self, value: bool) {
        self.index0 |= EntityMetadataIndex0::IsInvisible
    }

    fn set_on_fire(&mut self, value: bool) {
        self.index0 |= EntityMetadataIndex0::IsOnFire
    }

    fn set_sprinting(&mut self, value: bool) {
        self.index0 |= EntityMetadataIndex0::IsSprinting
    }

    fn set_swimming(&mut self, value: bool) {
        self.index0 |= EntityMetadataIndex0::IsSwimming
    }

    fn set_air_ticks(&mut self, value: i32) {
        self.air_ticks = value.into()
    }

    fn set_custom_name(&mut self, value: Option<Box<Chat>>) {
        self.custom_name = value
    }

    fn set_custom_name_visible(&mut self, value: bool) {
        self.is_custom_name_visible = value
    }

    fn set_silent(&mut self, value: bool) {
        self.is_silent = value
    }

    fn set_no_gravity(&mut self, value: bool) {
        self.has_no_gravity = value
    }

    fn set_pose(&mut self, value: Pose) {
        self.pose = value
    }

    fn set_tick_frozen_is_powdered_snow(&mut self, value: i32) {
        self.tick_frozen_is_powdered_snow = value.into()
    }
}

pub struct LivingEntityMetadata {
    entity: EntityMetadata,
    index8: LivingEntityIndex8,
    health: f32,
    potion_effect_color: VarInt,
    is_potion_effect_ambient: bool,
    number_of_arrows_in_entity: VarInt,
    number_of_bee_stingers_in_entity: VarInt,
    location_of_the_bed_that_the_entity_is_currently_sleeping_in: Option<Position>,
}

impl Default for LivingEntityMetadata {
    fn default() -> Self {
        Self {
            entity: Default::default(),
            index8: LivingEntityIndex8::default(),
            health: 0.0,
            potion_effect_color: 0.into(),
            is_potion_effect_ambient: false,
            number_of_arrows_in_entity: 0.into(),
            number_of_bee_stingers_in_entity: 0.into(),
            location_of_the_bed_that_the_entity_is_currently_sleeping_in: None,
        }
    }
}
pub trait LivingEntityMeta {
    fn active_hand(&self) -> bool;
    fn health(&self) -> f32;
    fn is_hand_active(&self) -> bool;
    fn is_in_riptide_spin_attack(&self) -> bool;
    fn is_potion_effect_ambient(&self) -> bool;
    fn location_of_the_bed_that_the_entity_is_currently_sleeping_in(&self) -> &Option<Position>;
    fn number_of_arrows_in_entity(&self) -> i32;
    fn number_of_bee_stingers_in_entity(&self) -> i32;
    fn potion_effect_color(&self) -> i32;
}

impl LivingEntityMetadata {
    delegate! {
        to self.entity {
            pub fn has_glowing_effect(&self) -> bool;
            pub fn is_crouching(&self) -> bool;
            pub fn is_flying_with_an_eytra(&self) -> bool;
            pub fn is_invisible(&self) -> bool;
            pub fn is_on_fire(&self) -> bool;
            pub fn is_sprinting(&self) -> bool;
            pub fn is_swimming(&self) -> bool;
            pub fn air_ticks(&self) -> i32;
            pub fn custom_name(&self) -> &Option<Box<Chat>>;
            pub fn is_custom_name_visible(&self) -> bool;
            pub fn is_silent(&self) -> bool;
            pub fn has_no_gravity(&self) -> bool;
            pub fn pose(&self) -> Pose;
            pub fn tick_frozen_is_powdered_snow(&self) -> i32;
        }
    }
}

impl LivingEntityMeta for LivingEntityMetadata {
    fn active_hand(&self) -> bool {
        self.index8.intersects(LivingEntityIndex8::ActiveHand)
    }

    fn health(&self) -> f32 {
        self.health
    }

    fn is_hand_active(&self) -> bool {
        self.index8.intersects(LivingEntityIndex8::IsHandActive)
    }

    fn is_in_riptide_spin_attack(&self) -> bool {
        self.index8
            .intersects(LivingEntityIndex8::IsInRiptideSpinAttack)
    }

    fn is_potion_effect_ambient(&self) -> bool {
        self.is_potion_effect_ambient
    }

    fn location_of_the_bed_that_the_entity_is_currently_sleeping_in(&self) -> &Option<Position> {
        &self.location_of_the_bed_that_the_entity_is_currently_sleeping_in
    }

    fn number_of_arrows_in_entity(&self) -> i32 {
        self.number_of_arrows_in_entity.into()
    }

    fn number_of_bee_stingers_in_entity(&self) -> i32 {
        self.number_of_bee_stingers_in_entity.into()
    }

    fn potion_effect_color(&self) -> i32 {
        self.potion_effect_color.into()
    }
}

bitflags! {
    pub struct LivingEntityIndex8 : u8 {
        const IsHandActive = 0x01;
        const ActiveHand = 0x02;
        const IsInRiptideSpinAttack = 0x04;
        const None = 0x00;
    }
}

impl Default for LivingEntityIndex8 {
    fn default() -> Self {
        LivingEntityIndex8::None
    }
}

pub trait PlayerMetadata {
    fn additional_hearts(&self) -> f32;
    fn score(&self) -> i32;
    fn cape_enabled(&self) -> bool;
    fn jacket_enabled(&self) -> bool;
    fn left_sleeve_enabled(&self) -> bool;
    fn right_sleeve_enabled(&self) -> bool;
    fn left_pants_leg_enabled(&self) -> bool;
    fn right_pants_leg_enabled(&self) -> bool;
    fn hat_enabled(&self) -> bool;
    fn main_hand(&self) -> MainHand;
}

pub struct Player {
    living_entity: LivingEntityMetadata,
    additional_hearts: f32,
    score: i32,
    player_byte0: PlayerByte0,
    main_hand: MainHand,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            living_entity: Default::default(),
            additional_hearts: Default::default(),
            score: Default::default(),
            player_byte0: PlayerByte0::None,
            main_hand: MainHand::Right,
        }
    }
}

impl PlayerMetadata for Player {
    fn additional_hearts(&self) -> f32 {
        self.additional_hearts
    }

    fn score(&self) -> i32 {
        self.score
    }

    fn cape_enabled(&self) -> bool {
        self.player_byte0.intersects(PlayerByte0::CapeEnabled)
    }

    fn jacket_enabled(&self) -> bool {
        self.player_byte0.intersects(PlayerByte0::JacketEnabled)
    }

    fn left_sleeve_enabled(&self) -> bool {
        self.player_byte0.intersects(PlayerByte0::LeftSleeveEnabled)
    }

    fn right_sleeve_enabled(&self) -> bool {
        self.player_byte0
            .intersects(PlayerByte0::RightSleeveEnabled)
    }

    fn left_pants_leg_enabled(&self) -> bool {
        self.player_byte0
            .intersects(PlayerByte0::LeftPantsLegEnabeld)
    }

    fn right_pants_leg_enabled(&self) -> bool {
        self.player_byte0
            .intersects(PlayerByte0::RightPantsLegEnabled)
    }

    fn hat_enabled(&self) -> bool {
        self.player_byte0.intersects(PlayerByte0::HatEnabled)
    }

    fn main_hand(&self) -> MainHand {
        self.main_hand
    }
}

bitflags! {
    pub struct PlayerByte0: u8 {
        const CapeEnabled = 0x01;
        const JacketEnabled = 0x02;
        const LeftSleeveEnabled = 0x04;
        const RightSleeveEnabled = 0x08;
        const LeftPantsLegEnabeld = 0x10;
        const RightPantsLegEnabled = 0x20;
        const HatEnabled = 0x40;
        const Unused = 0x80;
        const None = 0x00;
    }
}

impl Encoder for EntityMetadata {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.index0.0 .0)?;
        writer.write_var_i32(self.air_ticks.into())?;
        writer.write_option(&self.custom_name)?;
        writer.write_bool(self.is_custom_name_visible)?;
        writer.write_bool(self.is_silent)?;
        writer.write_bool(self.has_no_gravity)?;
        self.pose.encode_to_write(writer)?;
        writer.write_var_i32(self.tick_frozen_is_powdered_snow.into())?;
        Ok(())
    }
}

impl Encoder for LivingEntityMetadata {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.entity.encode_to_write(writer)?;
        Ok(())
    }
}

impl Encoder for Player {
    fn encode_to_write<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(())
    }
}

impl Metadata for Player {
    fn get_metadata_id(&self) -> i32 {
        todo!()
    }
}
