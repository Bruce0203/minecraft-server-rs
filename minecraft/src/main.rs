use common_server::selector::{ConnectionHandler, Selector};
use minecraft::server::Server;

fn main() {
    let addr = "0.0.0.0:25565".parse().unwrap();
    let server = Server::new(addr);
    let mut selector = Selector::new(addr, 256, server);
    selector.start_selection_loop(None);
}
