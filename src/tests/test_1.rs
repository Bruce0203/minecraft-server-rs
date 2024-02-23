use std::{
    io::{Read, Write},
    net::{Shutdown, SocketAddr},
    thread,
};

use mio::{
    event::Event,
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

struct Connections {
    inner: Vec<Connection>,
    index_queue: Vec<usize>,
}

impl Connections {
    fn new() -> Connections {
        Connections {
            inner: Vec::new(),
            index_queue: Vec::new(),
        }
    }

    fn add<F>(&mut self, mut connection: Connection, f: F)
    where
        F: FnOnce(usize, &mut Connection) -> (),
    {
        if let Some(index) = self.index_queue.pop() {
            f(index, &mut connection);
            self.inner[index] = connection;
        } else {
            let len = self.inner.len();
            f(len, &mut connection);
            self.inner.push(connection);
        }
    }

    fn remove(&mut self, index: usize) {
        self.index_queue.push(index);
    }

    fn get(&mut self, index: usize) -> &mut Connection {
        unsafe { self.inner.get_unchecked_mut(index) }
    }
}

#[test]
fn test_1() {
    let server_addr: SocketAddr = "127.0.0.1:25565".parse().unwrap();
    let mut listener = TcpListener::bind(server_addr).unwrap();

    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(128);
    let server_token = Token(usize::MAX);
    poll.registry()
        .register(&mut listener, server_token, Interest::READABLE)
        .unwrap();
    let mut connections = Connections::new();

    println!("server started!");
    start_bot_daemons(server_addr);

    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            let token = event.token();
            if token == server_token {
                println!("new accept omitted");
                if let Ok((mut stream, addr)) = listener.accept() {
                    let index = connections.add(
                        Connection {
                            stream: stream,
                            acc: 0,
                        },
                        |index, connection| {
                            poll.registry().register(
                                &mut connection.stream,
                                Token(index),
                                Interest::READABLE,
                            );
                        },
                    );
                    println!("new client token={:#?}", index);
                };
                continue;
            }
            let token_index = token.0;
            let mut buf = [0u8; 1000];
            let connection = connections.get(token_index);
            let read = connection.stream.read(&mut buf).unwrap();
            if read == 0 {
                println!("connection closed(token={:#?})", token_index);
                poll.registry().deregister(&mut connection.stream);
                connections.remove(token_index);
                continue;
            }
            let read_buf = &buf[0..read];
            println!("read: {:#?}", read_buf);
            connection.acc += 1;
            if connection.acc == 10 {
                println!("acc: {}", connection.acc);
            }
        }
    }
}

struct Connection {
    stream: TcpStream,
    acc: i32,
}

fn start_bot_daemons(server_addr: SocketAddr) {
    thread::spawn(move || {
        thread::sleep_ms(1000);
        println!("bot started!");
        for i in 0..10 {
            let mut client = std::net::TcpStream::connect(server_addr).unwrap();
            for i in 0..1 {
                client.write(&[1, 2, 3]).unwrap();
            }
            thread::sleep_ms(500);
            client.flush();
            client.shutdown(Shutdown::Both);
        }
    });
}
