use backtrace::Backtrace;
use common_server::selector::Selector;

use crate::server::Server;

#[test]
fn test_backtrace() {
    println!("stack depth={}", stack_frame_depth());
}

#[inline]
fn stack_frame_depth() -> usize {
    Backtrace::new_unresolved().frames().len()
}

#[test]
fn test_handshake_server() {
    println!("Server started!");
    let mut server = Server::new();
    let mut selector = Selector::bind("127.0.0.1:25555".parse().unwrap(), 256);
    selector.start_selection_loop(&mut server);
}


