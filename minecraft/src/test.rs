#[test]
fn handshake_server() {
    use common_server::selector::Selector;
    use crate::server::Server;

    let mut server = Server::new();
    let mut selector = Selector::bind("127.0.0.1:25565".parse().unwrap(), &mut server, 256);
    selector.start_selection_loop(None);
}
