use crate::server::prelude::{LoginPlayer, LoginServer};

use super::v1_20_4::v1_20_4::V1_20_4;

pub struct Minecraft;

super::protocols!(LoginServer, LoginPlayer, V1_20_4, V1_20_4,);
