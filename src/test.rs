use crate::{server::prelude::Server, net::prelude::Selector};

#[test]
fn test_mc_server() {
    Selector {
        server: Server::new(),
    }
    .run();
}

