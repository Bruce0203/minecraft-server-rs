use server_workspace::{net::prelude::Selector, server::prelude::Server};

fn main() {
    Selector {
        server: Server::new(),
    }
    .run::<100_000>();
}
