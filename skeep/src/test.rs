use crate::packet::status::StatusRequest;

#[test]
#[ignore]
fn test_handshake_server() {
    use std::{io::Read, thread};

    use common_server::{encoder::Encoder, selector::Selector};

    use crate::{packet::handshake::HandShake, server::Server};

    let addr = "127.0.0.1:25566".parse().unwrap();
    let mut selector = Selector::bind(addr, 256);
    let mut server = Server::new();
    thread::spawn(move || {
        selector.start_selection_loop(&mut server);
    });

    println!("start!test");
    for i in 0..10 {
        let mut stream = std::net::TcpStream::connect(addr).unwrap();
        println!("connected!");
        use std::io::Write;
        stream
            .write_all(
                HandShake::new(1, crate::packet::handshake::NextState::Status)
                    .encode()
                    .as_mut(),
            )
            .unwrap();
        //stream.write_all(StatusRequest::new()).unwrap();
        let mut buf = vec![];
        let read = stream.read(&mut buf).unwrap();
        let read_buf = &buf[0..read];
        assert_eq!(read_buf[0], i + 1);
    }
    println!("Done");
}
