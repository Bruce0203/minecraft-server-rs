use socket_selector::Selector;

#[test]
fn test_my_server() {
    use crate::server::Server;
    let addr = "0.0.0.0:25565".parse().unwrap();
    let server = Server::new();
    let selector = Selector::new::<256>(addr, server);
    selector.start_selection_loop::<10000>(None);
}
