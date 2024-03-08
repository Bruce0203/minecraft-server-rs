use minecraft_server::{net::prelude::Selector, server::prelude::Server};

fn main() {
    Selector {
        server: Server::new(),
    }
    .run::<100_000>();
}
