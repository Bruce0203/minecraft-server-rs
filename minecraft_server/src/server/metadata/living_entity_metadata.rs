use super::prelude::EntityMetadata;

pub struct LivingEntityMetadata(EntityMetadata<14>);
pub struct LivingEntity(EntityMetadata<14>);

impl LivingEntity {
    pub fn new() -> LivingEntity {
        LivingEntity(EntityMetadata::<14>::new())
    }
}


