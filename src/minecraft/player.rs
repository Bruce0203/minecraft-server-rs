use derivative::Derivative;

use super::session_relay::SessionRelay;

#[derive(Default)]
pub struct Player {
    pub session_relay: SessionRelay
}
