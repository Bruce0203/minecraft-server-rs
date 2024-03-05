use crate::server::prelude::Server;

#[test]
fn test_mc_server() {
    Server::new().run();
}
