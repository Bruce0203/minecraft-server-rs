use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Default)]
pub struct SessionRelay {
    pub encryption_enabled: bool,
    #[derivative(Default(value = "-1"))]
    pub compression_threshold: i32,
    pub protocol_id: i32,
    pub connection_state: ConnState,
}

#[derive(Debug, Default)]
pub enum ConnState {
    #[default]
    HandShake,
    Status,
    Login,
    Confgiuration,
    Play,
    Closed,
}

#[test]
fn derivative_test() {
    assert_eq!(SessionRelay::default().compression_threshold, -1);
}
