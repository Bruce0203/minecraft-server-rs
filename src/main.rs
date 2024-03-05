use server_workspace::server::server::{Selector, Server};

fn main() {
    Selector {
        server: Server::new(),
    }
    .run();
}
