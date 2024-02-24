use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
use std::{
    io::{Read, Write},
    net::SocketAddr,
    thread::{self},
};

struct Socket<T> {
    stream: TcpStream,
    token: usize,
    addr: SocketAddr,
    connection: Box<T>,
}

impl<T> Socket<T> {
    fn register_and_new(
        mut stream: TcpStream,
        token: usize,
        addr: SocketAddr,
        connection: Box<T>,
        poll: &mut Poll,
    ) -> Socket<T> {
        //todo check when result throw err
        poll.registry()
            .register(&mut stream, Token(token), Interest::READABLE)
            .unwrap();
        Socket {
            stream,
            token,
            addr,
            connection,
        }
    }
}

type IndexQueue = Vec<usize>;
type Connections<T> = Vec<Option<Socket<T>>>;

struct SocketSelector<T> {
    listener: TcpListener,
    poll: Poll,
    connections: Connections<T>,
    index_queue: IndexQueue,
}

impl<T: Default> SocketSelector<T> {
    pub fn bind(addr: SocketAddr, max_connection_pool: usize) -> SocketSelector<T> {
        SocketSelector {
            listener: TcpListener::bind(addr).unwrap(),
            poll: Poll::new().unwrap(),
            connections: Connections::with_capacity(max_connection_pool),
            index_queue: IndexQueue::with_capacity(max_connection_pool),
        }
    }

    fn accept_socket<T2: Server<T>>(&mut self, socket_server: &mut T2) {
        if let Ok((stream, addr)) = self.listener.accept() {
            let index = 1usize;
            macro_rules! new_socket {
                ($index:expr, $poll:expr, $stream:expr) => {
                    Socket::<T>::register_and_new(
                        $stream,
                        $index,
                        addr,
                        Box::new(socket_server.new_connection()),
                        $poll,
                    );
                };
            }
            if let Some(index) = self.index_queue.pop() {
                self.connections[index] = Some(new_socket!(index, &mut self.poll, stream));
            } else {
                let index = self.connections.len();
                self.connections
                    .push(Some(new_socket!(index, &mut self.poll, stream)));
            }
        }
    }

    fn handle_socket_read<T2: Server<T>>(
        &mut self,
        socket_server: &mut T2,
        token_index: usize,
        buf: &mut [u8],
    ) {
        let connection = unsafe { self.connections.get_unchecked_mut(token_index) }
            .as_mut()
            .unwrap();
        let read = connection.stream.read(buf).unwrap();
        if read == 0 {
            self.poll
                .registry()
                .deregister(&mut connection.stream)
                .unwrap();
            self.index_queue.push(token_index);
            self.connections[token_index] = None;
        } else {
            let mut read_buf = &buf[0..read];
            socket_server.handle_connection_read(read_buf);
        }
    }
}

pub trait Server<T: Default>: Sized {
    fn new_connection(&mut self) -> T;
    fn handle_connection_read(&mut self, buf: &[u8]);

    fn start_selection_loop(
        &mut self,
        addr: SocketAddr,
        events_capacity: usize,
        max_connection_pool: usize,
    ) {
        let mut selector = SocketSelector::<T>::bind(addr, max_connection_pool);
        let server_token = Token(usize::MAX);
        selector
            .poll
            .registry()
            .register(&mut selector.listener, server_token, Interest::READABLE)
            .unwrap();
        let mut buf = [0u8; 1000];
        let mut events = Events::with_capacity(events_capacity);
        loop {
            selector.poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                let token = event.token();
                if token == server_token {
                    selector.accept_socket(self);
                } else {
                    let token_index = token.0;
                    selector.handle_socket_read(self, token_index, &mut buf);
                }
            }
        }
    }
}
struct HandShakeSelector {}

#[test]
fn test_1() {
    struct Player;
    impl Default for Player {
        fn default() -> Self {
            Self {}
        }
    }
    struct MyServer {}
    impl Server<Player> for MyServer {
        fn new_connection(&mut self) -> Player {
            Player {}
        }

        fn handle_connection_read(&mut self, buf: &[u8]) {
            println!("read: {:#?}", buf);
        }
    }
    let addr = "127.0.0.1:25565".parse().unwrap();
    start_bot_daemons(addr);
    let server = MyServer {}.start_selection_loop(addr, 256, 256);
}

fn start_bot_daemons(server_addr: SocketAddr) {
    thread::spawn(move || {
        println!("bot started!");
        let mut i = 0;
        let mut vec = Vec::<std::net::TcpStream>::new();
        while i != 10 {
            if let Ok(mut client) = std::net::TcpStream::connect(server_addr) {
                for i in 0..1 {
                    client.write_all(&[i, i, i]).unwrap();
                }
                vec.push(client);
                thread::sleep_ms(1000);
                i += 1;
            };
        }
        for ele in vec {
            drop(ele);
            thread::sleep_ms(1000);
        }
    });
}
