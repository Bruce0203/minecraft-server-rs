use common_server::selector::Selector;

#[test]
fn test_my_server() {
    use crate::server::Server;
    let addr = "0.0.0.0:25565".parse().unwrap();
    let server = Server::new();
    let selector = Selector::new(addr, 256, server);
    selector.start_selection_loop(None);
}
