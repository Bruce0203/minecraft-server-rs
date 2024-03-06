use server_workspace::{server::server::{Server}, net::prelude::Selector};

fn main() {
    Selector {
        server: Server::new(),
    }
    .run();
}
