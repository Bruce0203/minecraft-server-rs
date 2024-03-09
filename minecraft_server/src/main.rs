use minecraft_server::{net::prelude::Selector, server::prelude::LoginServer};

fn main() {
    LoginServer::new().run();
}
