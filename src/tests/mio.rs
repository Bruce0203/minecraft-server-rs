use std::error::Error;
use std::thread::{self, spawn};

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

// Some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(100);
const CLIENT: Token = Token(1);

#[test]
#[ignore]
fn test_mio_is_ok() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the server socket.
    let addr = "127.0.0.1:13265".parse()?;
    let mut server: TcpListener = TcpListener::bind(addr)?;
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Setup the client socket.
    let mut client = TcpStream::connect(addr)?;
    // Register the socket.
//    poll.registry()
//        .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    thread::spawn(move || {
        for i in 0..10 {
            let stream = TcpStream::connect(addr);
        }
    });
    // Start an event loop.
    loop {
        // Poll Mio for events, blocking until we get an event.
        println!("start polling");

        poll.poll(&mut events, None)?;
        println!("end polling");

        // Process each event.
        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            println!("omitted events!");

            println!("{}", event.token().0);
            match event.token() {
                SERVER => {
                    // If this is an event for the server, it means a connection
                    // is ready to be accepted.
                    //
                    // Accept the connection and drop it immediately. This will
                    // close the socket and notify the client of the EOF.
                    let connection = server.accept();
                    //drop(connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        // We can (likely) write to the socket without blocking.
                    }

                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                    }

                    // Since the server just shuts down the connection, let's
                    // just exit from our event loop.
                    //return Ok(());
                }
                // We don't expect any events with tokens other than those we provided.
                _ => unreachable!(),
            }
        }
    }
}
