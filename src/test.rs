use crate::{net::prelude::Selector, server::prelude::Server};

#[test]
fn test_mc_server() {
    Selector {
        server: Server::new(),
    }
    .run::<100_000>();
}
