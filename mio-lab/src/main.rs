use mio::net::TcpListener;
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use std::io;
use std::ops::BitOr;

const SERVER: Token = Token(0);

const DATA: &[u8] = b"Hello world!\n";

fn main() -> io::Result<()> {
    let mut poll = Poll::new()?;
    let mut events: Events = Events::with_capacity(128);

    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut server = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE | Interest::WRITABLE)?;

    loop {
        poll.poll(&mut events, None)?;
        println!("{}", "loop");
        for event in events.iter() {
            println!("{}", event.token().0);
            let connection = server.accept();
            drop(connection);
            break;
        }
    }
}
