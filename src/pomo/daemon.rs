use std::{
    fs,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    sync::mpsc::{self, Sender},
    thread,
};

use bincode;

use crate::countdown::countdown;
use crate::types::{Message};

pub fn run_daemon(socket_path: &str) -> std::io::Result<()> {
    // 1. Create new lock to prevent spawning new daemons
    // 2. Create unix socket
    // 3. Spawn new countdown process in new thread
    // 4. Listen to messages at unix socket and pass them to our countdown timer
    // 5. Release lock on shutdown (kill)
    let socket_path = Path::new(socket_path);

    if socket_path.exists() {
        fs::remove_file(socket_path)?;
    }

    // channels
    let (msg_tx, msg_rx) = mpsc::channel::<Message>();

    // spawn countdown
    thread::spawn(move || countdown(msg_rx));

    // Unix socket listner
    let listner = UnixListener::bind(&socket_path)?;
    listner.set_nonblocking(true)?;

    // daemon loop
    loop {
        // listens on open unix socket for messages
        match listner.accept() {
            Ok((stream, _)) => handle_stream(stream, msg_tx.clone())?,
            Err(e) => match e.kind() {
                std::io::ErrorKind::WouldBlock => (),
                _ => return Err(e),
            },
        }
    }
}

// handle incoming messages from unix socket and send to msg_tx
pub fn handle_stream(mut stream: UnixStream, msg_tx: Sender<Message>) -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    stream.read_to_end(&mut buffer)?;
    let msg: Message = bincode::deserialize(&buffer).unwrap();
    msg_tx.send(msg).unwrap();

    Ok(())
}

// send message to unix socket
pub fn send_message(socket_path: &str, msg: Message) -> std::io::Result<()> {
    let socket_path = Path::new(socket_path);
    let mut stream = UnixStream::connect(socket_path)?;
    let serialized = bincode::serialize(&msg).unwrap();
    stream.write_all(&serialized)?;
    Ok(())
}
