use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Result, Write},
    net::SocketAddr,
};

pub struct Socket<T> {
    pub stream: TcpStream,
    pub token: usize,
    pub addr: SocketAddr,
    pub connection: Box<T>,
}

impl<T> Socket<T> {
    fn new_and_register(
        mut stream: TcpStream,
        token: usize,
        addr: SocketAddr,
        connection: Box<T>,
        poll: &mut Poll,
    ) -> Result<Socket<T>> {
        poll.registry()
            .register(&mut stream, Token(token), Interest::READABLE)?;
        Ok(Socket {
            stream,
            token,
            addr,
            connection,
        })
    }
}

type IndexQueue = Vec<usize>;
type IndexedConnections<T> = Vec<Option<Socket<T>>>;

pub struct Selector<T> {
    pub listener: TcpListener,
    poll: Poll,
    indexed_connection: IndexedConnections<T>,
    index_queue: IndexQueue,
}

impl<T> Selector<T> {
    pub fn bind(addr: SocketAddr, max_connection_pool: usize) -> Selector<T> {
        Selector {
            listener: TcpListener::bind(addr).unwrap(),
            poll: Poll::new().unwrap(),
            indexed_connection: IndexedConnections::with_capacity(max_connection_pool),
            index_queue: IndexQueue::with_capacity(max_connection_pool),
        }
    }

    pub fn close_connection(&mut self, socket: &mut Socket<T>) -> std::io::Result<()>{
        self.poll.registry().deregister(&mut socket.stream)?;

        Ok(())
    }

    pub fn start_selection_loop<T2>(&mut self, connection_handler: &mut T2)
    where
        T2: ConnectionHandler<T>,
    {
        let server_token = Token(usize::MAX);
        self.poll
            .registry()
            .register(&mut self.listener, server_token, Interest::READABLE)
            .unwrap();
        const MAX_READ_BUFFER_SIZE: usize = 2097151;
        let mut buf = [0u8; MAX_READ_BUFFER_SIZE];
        let events_capacity = 128;
        let mut events = Events::with_capacity(events_capacity);
        loop {
            println!("poll");
            #[warn(unused_must_use)]
            self.poll.poll(&mut events, None);
            println!("endpoll");
            for event in events.iter() {
                let token = event.token();
                if token == server_token {
                    self.accept_socket(connection_handler);
                } else {
                    let token_index = token.0;
                    self.handle_socket_read(connection_handler, token_index, &mut buf);
                }
            }
            self.debug_selector();
        }
    }

    fn accept_socket<T2: ConnectionHandler<T>>(&mut self, socket_server: &mut T2) {
        if let Ok((stream, addr)) = self.listener.accept() {
            macro_rules! new_socket {
                ($index:expr, $poll:expr, $stream:expr) => {
                    Socket::<T>::new_and_register(
                        $stream,
                        $index,
                        addr,
                        Box::new(socket_server.handle_connection_accept()),
                        $poll,
                    )
                };
            }
            if let Some(index) = self.index_queue.pop() {
                if let Ok(socket) = new_socket!(index, &mut self.poll, stream) {
                    self.indexed_connection[index] = Some(socket);
                }
            } else {
                let index = self.indexed_connection.len();
                if let Ok(socket) = new_socket!(index, &mut self.poll, stream) {
                    self.indexed_connection.push(Some(socket));
                }
            }
        }
    }

    fn handle_socket_read<T2: ConnectionHandler<T>>(
        &mut self,
        socket_server: &mut T2,
        token_index: usize,
        buf: &mut [u8],
    ) -> Result<()> {
        let socket_result =
            unsafe { self.indexed_connection.get_unchecked_mut(token_index) }.as_mut();

        if socket_result.is_none() {
            println!("socket is none");
            panic!();
        }
        let socket = socket_result.unwrap();

        let read = socket.stream.read(buf)?;
        if read == 0 {
            self.poll.registry().deregister(&mut socket.stream)?;
            socket_server.handle_connection_closed(socket);
            self.index_queue.push(token_index);
            self.indexed_connection[token_index] = None;
        } else {
            let read_buf = &buf[0..read];
            socket_server.handle_connection_read(socket, read_buf);
        }
        Ok(())
    }

    fn debug_selector(&mut self) {
        return;
        println!(
            "{:#?}",
            self.indexed_connection
                .iter()
                .map(|mut conn| {
                    if let Some(conn) = conn.as_ref() {
                        conn.addr.to_string()
                    } else {
                        "none".to_string()
                    }
                })
                .fold(String::new(), |acc, elem| acc + &elem + ", ")
                .to_string()
        );
    }
}

pub trait ConnectionHandler<T: Sized>: Sized {
    fn handle_connection_accept(&mut self) -> T;
    fn handle_connection_read(&mut self, socket: &mut Socket<T>, buf: &[u8]);
    fn handle_connection_closed(&mut self, socket: &mut Socket<T>);
}

#[test]
#[ignore]
fn test_selector() {
    struct Player {
        acc: i32,
    }

    impl Default for Player {
        fn default() -> Self {
            Self { acc: 0 }
        }
    }

    struct MyServer {}

    impl ConnectionHandler<Player> for MyServer {
        fn handle_connection_accept(&mut self) -> Player {
            println!("socket accepted!");
            Player::default()
        }

        fn handle_connection_read(&mut self, socket: &mut Socket<Player>, buf: &[u8]) {
            socket.connection.acc += buf[0] as i32;
        }

        fn handle_connection_closed(&mut self, socket: &mut Socket<Player>) {
            println!("socket closed!");
        }
    }

    let addr = "127.0.0.1:25565".parse().unwrap();
    start_bot_daemons(addr);
    let mut selector = Selector::bind(addr, 256);
    let mut server = MyServer {};
    selector.start_selection_loop(&mut server);

    fn start_bot_daemons(server_addr: SocketAddr) {
        use std::{thread, time::Duration};

        thread::sleep(Duration::from_millis(500));
        thread::spawn(move || {
            println!("bot started!");
            let mut i = 0;
            let mut vec = Vec::<std::net::TcpStream>::new();
            while i != 11 {
                if let Ok(mut client) = std::net::TcpStream::connect(server_addr) {
                    for i in 0..1 {
                        std::io::Write::write_all(&mut client, &[i, i, i]).unwrap();
                    }
                    vec.push(client);
                    i += 1;
                };
            }
            for mut client in vec {
                client.flush().unwrap();
            }
            thread::sleep(Duration::from_secs(1000));
        });
    }
}
