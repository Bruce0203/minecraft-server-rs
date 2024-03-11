use crate::{net::prelude::Selector, server::prelude::GameServer};

#[test]
fn test_mc_server() {
    GameServer::new().run::<100_000>();
}
