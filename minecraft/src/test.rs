use common_server::selector::{ConnectionHandler, Selector};

#[test]
fn test_my_server() {
    use crate::server::Server;
    let addr = "0.0.0.0:25565".parse().unwrap();
    let server = Server::new(addr);
    let selector = Selector::new(addr, 256, server);
    selector.start_selection_loop(None);
}
