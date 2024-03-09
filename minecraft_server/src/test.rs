use crate::{net::prelude::Selector, protocol::v1_20_4::v1_20_4::HandShakeServer, server::prelude::LoginServer};

#[test]
fn test_mc_server() {
    let mut login_server = HandShakeServer::new();
    login_server.run();
}
