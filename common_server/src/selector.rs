use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
use std::{
    io::{Read, Result},
    net::SocketAddr,
    time::Duration,
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

pub struct Selector<'a, T, T2: ConnectionHandler<T>> {
    pub listener: TcpListener,
    poll: Poll,
    indexed_connection: IndexedConnections<T>,
    index_queue: IndexQueue,
    connection_handler: &'a mut T2,
}

impl<'a, T, T2: ConnectionHandler<T>> Selector<'a, T, T2> {
    pub fn bind(
        addr: SocketAddr,
        connection_handler: &mut T2,
        max_connection_pool: usize,
    ) -> Selector<T, T2> {
        Selector {
            listener: TcpListener::bind(addr).unwrap(),
            poll: Poll::new().unwrap(),
            indexed_connection: IndexedConnections::with_capacity(max_connection_pool),
            index_queue: IndexQueue::with_capacity(max_connection_pool),
            connection_handler,
        }
    }

    pub fn close_connection(&mut self, socket: &mut Socket<T>) -> std::io::Result<()> {
        self.poll.registry().deregister(&mut socket.stream)?;
        self.connection_handler.handle_connection_closed(socket);
        self.index_queue.push(socket.token);
        self.indexed_connection[socket.token] = None;
        Ok(())
    }

    pub fn start_selection_loop(&mut self, timeout: Option<Duration>) {
        let server_token = Token(usize::MAX);
        self.poll
            .registry()
            .register(&mut self.listener, server_token, Interest::READABLE)
            .unwrap();
        const MAX_READ_BUFFER_SIZE: usize = 10000;
        let mut buf = [0u8; MAX_READ_BUFFER_SIZE];
        let events_capacity = 128;
        let mut events = Events::with_capacity(events_capacity);
        loop {
            #[warn(unused_must_use)]
            if let Err(_) = self.poll.poll(&mut events, timeout) {
                continue;
            }
            self.connection_handler.handle_update();
            for event in events.iter() {
                let token = event.token();
                if token == server_token {
                    self.accept_socket();
                } else {
                    let token_index = token.0;
                    if let Err(_) = self.handle_socket_read(token_index, &mut buf) {
                    }
                }
            }
            self.debug_selector();
        }
    }

    fn accept_socket(&mut self) {
        if let Ok((stream, addr)) = self.listener.accept() {
            macro_rules! new_socket {
                ($index:expr, $poll:expr, $stream:expr) => {
                    Socket::<T>::new_and_register(
                        $stream,
                        $index,
                        addr,
                        Box::new(self.connection_handler.handle_connection_accept()),
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

    fn handle_socket_read(&mut self, token_index: usize, buf: &mut [u8]) -> Result<()> {
        let socket_result =
            unsafe { (self.indexed_connection).get_unchecked_mut(token_index) }.as_mut();

        if socket_result.is_none() {
            println!("socket is none");
            panic!();
        }

        let socket = socket_result.unwrap();
        let read = socket.stream.read(buf)?;
        if read == 0 {
            self.poll.registry().deregister(&mut socket.stream)?;
            self.connection_handler.handle_connection_closed(socket);
            self.index_queue.push(token_index);
            self.indexed_connection[token_index] = None;
        } else {
            let read_buf = &buf[0..read];
            self.connection_handler
                .handle_connection_read(socket, read_buf)?;
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
    fn handle_connection_read(&mut self, socket: &mut Socket<T>, buf: &[u8]) -> Result<()>;
    fn handle_connection_closed(&mut self, socket: &mut Socket<T>);
    fn handle_update(&mut self);
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

        fn handle_connection_read(
            &mut self,
            socket: &mut Socket<Player>,
            buf: &[u8],
        ) -> Result<()> {
            socket.connection.acc += buf[0] as i32;
            Ok(())
        }

        fn handle_connection_closed(&mut self, socket: &mut Socket<Player>) {
            println!("socket closed!");
        }

        fn handle_update(&mut self) {}
    }

    let addr = "127.0.0.1:25565".parse().unwrap();
    start_bot_daemons(addr);
    let mut server = MyServer {};
    let mut selector = Selector::bind(addr, &mut server, 256);
    selector.start_selection_loop(None);

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
                std::io::Write::flush(&mut client).unwrap();
            }
            thread::sleep(Duration::from_secs(1000));
        });
    }
}
