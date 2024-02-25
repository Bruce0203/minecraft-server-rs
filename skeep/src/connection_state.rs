use std::default;

use derivative::Derivative;

#[derive(Debug, Default)]
pub enum ConnectionState {
    #[default]
    HandShake,
    Status,
    Login,
    Configuration,
    Play,
    Closed,
}
