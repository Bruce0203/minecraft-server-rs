use crate::{net::prelude::Selector, server::prelude::LoginServer};

#[test]
fn test_mc_server() {
    LoginServer::new().run::<100>();
}
