use std::io::{prelude::Write, Result};

use crate::io::prelude::{Encoder, U16Write, U8Write, VarIntWrite};

#[derive(Debug)]
pub struct SetContainerProperty {
    window_id: u8,
    property: u16,
    value: u16,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Furnace {
    FireIcon,
    MaximumFuelBurnTime,
    ProgressArrow,
    MaximumProgress,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Beacon {
    PowerLevel,
    FirstPotionEffect,
    SecondPotionEffect,
}

#[derive(Debug)]
pub enum Anvil {
    RepairCost,
}

#[derive(Debug)]
pub enum BrewingStand {
    BrewTime,
    FuelTime,
}

#[derive(Debug)]
pub enum Stonecutter {
    SelectedRecipe,
}

#[derive(Debug)]
pub enum Loom {
    SelectedPattern,
}

#[derive(Debug)]
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
