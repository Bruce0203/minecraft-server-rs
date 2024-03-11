use std::io::{prelude::Write, Result};

use crate::io::prelude::{Encoder, U16Write, U8Write, VarIntWrite};

pub struct SetContainerProperty {
    window_id: u8,
    property: u16,
    value: u16,
}

pub enum Property {
    Furnace(Furnace),
    EnchantmentTable(EnchantmentTable),
    Beacon(Beacon),
    Anvil(Anvil),
    BrewingStand(BrewingStand),
    Stonecutter(Stonecutter),
    Loom(Loom),
    Lectern(Lectern),
}

pub enum Furnace {
    FireIcon,
    MaximumFuelBurnTime,
    ProgressArrow,
    MaximumProgress,
}

pub enum EnchantmentTable {
    LevelRequirementForTopEnchantmentSlot,
    LevelRequirementForMiddleEnchantmentSlot,
    TheEnchantmentSeed,
    EnchantmentIDShownOnMouseHoverOverTopEnchantmentSlot,
    EnchantmentIDShownOnMouseHOverOverMiddleEnchantmentSlot,
    EnchantmentIDShownOnMouseHoverOverBottomEnchantmentSlot,
    EnchantmentLevelShownOnMouseHoverOverTopEnchantmentSlot,
    EnchantmentLevelShownOnMouseHOverOverMiddleEnchantmentSlot,
    EnchantmentLevelShownOnMouseHoverOverBottomEnchantmentSlot,
}

pub enum Beacon {
    PowerLevel,
    FirstPotionEffect,
    SecondPotionEffect,
}

pub enum Anvil {
    RepairCost,
}

pub enum BrewingStand {
    BrewTime,
    FuelTime,
}

pub enum Stonecutter {
    SelectedRecipe,
}

pub enum Loom {
    SelectedPattern,
}

pub enum Lectern {
    PageNumber,
}

impl Encoder for SetContainerProperty {
    fn encode_to_buffer(&self, buf: &mut crate::io::prelude::Buffer) -> Result<()> {
        buf.write_u8(self.window_id)?;
        buf.write_u16(self.property)?;
        buf.write_u16(self.value)?;
        Ok(())
    }
}
