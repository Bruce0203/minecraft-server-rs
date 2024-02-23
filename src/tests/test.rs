use std::{
    error::Error,
    io::{Write},
    net::{SocketAddr},
};

use mio::{event, Events, Interest, Poll, Token, net::TcpListener};

struct Connection {
    stream: TcpStream,
}

#[test]
#[ignore]
fn start_select() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(1);
    let addr = "127.0.0.1:12344".parse()?;

    let mut listener = TcpListener::bind(addr)?;
    let listener_token = Token(100);
    poll.registry().register(
        &mut listener,
        listener_token,
        Interest::READABLE | Interest::WRITABLE | Interest::AIO,
    )?;

    let mut clients = Vec::<Connection>::with_capacity(10000);

    thread::spawn(move || {
        for i in 0..10 {
            if start_client(addr).is_ok() {
                println!("cilent initiated!");
            };
        }
    });
    loop {
        poll.poll(&mut events, None)?;
        println!("asfdqwexzcv");
        for event in events.iter() {
            let token = event.token();
            let token_index = token.0;
            if token == listener_token {
                if let Ok(stream) = listener.accept() {
                    clients.push(Connection { stream: stream.0 })
                }
            } else {
                let connection = clients.get_mut(token_index).unwrap();
                let mut buf = Vec::<u8>::with_capacity(10000);
                connection.stream.read_exact(&mut buf)?;
                println!("{:#?}", buf);
            }
        }
    }
}

fn start_client(addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(&vec![1, 2, 3, 4])?;
    Ok(())
}
