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

struct Connection {
    stream: TcpStream,
    acc: i32,
}

fn run(addr: SocketAddr) {
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(256);
    let mut connections = Vec::<Connection>::with_capacity(256);
    let mut index_queue = Vec::<usize>::with_capacity(256);
    let mut listener = TcpListener::bind(addr).unwrap();
    let server_token = Token(usize::MAX);
    poll.registry()
        .register(&mut listener, server_token, Interest::READABLE)
        .unwrap();

    loop {
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            let token = event.token();
            let token_index = token.0;
            if token_index == usize::MAX {
                if let Ok((mut stream, addr)) = listener.accept() {
                    let mut connection = Connection { stream, acc: 0 };
                    let index = if let Some(index) = index_queue.pop() {
                        poll.registry().register(
                            &mut connection.stream,
                            Token(index),
                            Interest::READABLE,
                        );
                        connections[index] = connection;
                        index
                    } else {
                        let len = connections.len();
                        poll.registry().register(
                            &mut connection.stream,
                            Token(len),
                            Interest::READABLE,
                        );
                        connections.push(connection);
                        len
                    };
                }
            } else {
                let mut buf = [0u8; 1000];
                let connection = unsafe { connections.get_unchecked_mut(token_index) };
                let read = connection.stream.read(&mut buf).unwrap();
                if read == 0 {
                    poll.registry().deregister(&mut connection.stream);
                    index_queue.push(token_index);
                    continue;
                }
                let read_buf = &buf[0..read];
                println!("read: {:#?}", read_buf);
            }
        }
    }
}

#[test]
fn test_1() {
    println!("server started!");
    let addr = "127.0.0.1:25565".parse().unwrap();
    start_bot_daemons(addr);
    run(addr);
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
