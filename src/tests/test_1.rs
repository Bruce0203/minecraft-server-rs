use mio::{
    event::Event,
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};
use std::{
    io::{Read, Write},
    net::{Shutdown, SocketAddr},
    thread::{self, sleep_ms},
};

struct Connection {
    stream: TcpStream,
    token: Token,
    addr: SocketAddr,
    acc: i32,
}

type IndexQueue = Vec<usize>;
type Connections = Vec<Option<Connection>>;

fn start_selector(addr: SocketAddr) -> ! {
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(256);
    let mut connections = Vec::<Option<Connection>>::with_capacity(256);
    let mut index_queue = Vec::<usize>::with_capacity(256);
    let mut listener = TcpListener::bind(addr).unwrap();
    let server_token = Token(usize::MAX);
    poll.registry()
        .register(&mut listener, server_token, Interest::READABLE)
        .unwrap();

    let mut buf = [0u8; 1000];
    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            let token = event.token();
            if token == server_token {
                accept_connection(&mut poll, &mut listener, &mut index_queue, &mut connections);
            } else {
                let token_index = token.0;
                handle_connection_read(
                    &mut poll,
                    &mut buf,
                    token_index,
                    &mut index_queue,
                    &mut connections,
                );
            }
            println!("------------");
            println!(
                "{:#?}",
                connections
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
}

fn accept_connection(
    poll: &mut Poll,
    listener: &mut TcpListener,
    index_queue: &mut IndexQueue,
    connections: &mut Connections,
) {
    if let Ok((stream, addr)) = listener.accept() {
        if let Some(index) = index_queue.pop() {
            let mut connection = add_client(Token(index), stream, addr);
            poll.registry()
                .register(&mut connection.stream, Token(index), Interest::READABLE)
                .unwrap();
            connections[index] = Some(connection);
            index
        } else {
            let len = connections.len();
            let mut connection = add_client(Token(len), stream, addr);
            poll.registry()
                .register(&mut connection.stream, Token(len), Interest::READABLE)
                .unwrap();
            connections.push(Some(connection));
            len
        };
    }
}

fn handle_connection_read(
    poll: &mut Poll,
    buf: &mut [u8],
    token_index: usize,
    index_queue: &mut IndexQueue,
    connections: &mut Connections,
) {
    let mut connection = unsafe { connections.get_unchecked_mut(token_index) }
        .as_mut()
        .unwrap();
    let read = connection.stream.read(buf).unwrap();
    if read == 0 {
        poll.registry().deregister(&mut connection.stream).unwrap();
        index_queue.push(token_index);
        println!("dropped: {:#?}", connection.addr.to_string());
        connections[token_index] = None;
    } else {
        let read_buf = &buf[0..read];
        //println!("read: {:#?}", read_buf);
    }
}

fn add_client(token: Token, stream: TcpStream, addr: SocketAddr) -> Connection {
    Connection {
        stream,
        token,
        addr,
        acc: 0,
    }
}

#[test]
fn test_1() {
    println!("server started!");
    let addr = "127.0.0.1:25565".parse().unwrap();
    start_bot_daemons(addr);
    start_selector(addr);
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
