use minecraft_server::{net::prelude::Selector, server::prelude::GameServer};

fn main() {
    GameServer::new().run::<100>();
}
