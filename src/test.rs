use crate::server::{prelude::Server, server::Selector};

#[test]
fn test_mc_server() {
    Selector {
        server: Server::new(),
    }
    .run();
}
