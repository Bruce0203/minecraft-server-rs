use minecraft_server::{
    net::prelude::{Selector, SocketSelector},
    server::prelude::GameServer,
};

fn main() {
    let server = GameServer::new();
    let mut selector = SocketSelector::new(server);
    selector.run_with_listener("0.0.0.0:25565".parse().unwrap());
}
