use common_server::selector::Selector;
use minecraft::server::Server;

fn main() {
    println!("Server started!");
    let mut server = Server::new();
    let mut selector = Selector::bind("127.0.0.1:25565".parse().unwrap(), 256);
    selector.start_selection_loop(&mut server);
}
