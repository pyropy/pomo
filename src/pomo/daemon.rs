use std::{
    fs,
    io::{Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::{Path, PathBuf},
    sync::mpsc::{self, Sender},
    thread,
};

use bincode;
use lockfile::Lockfile;

use crate::types::Message;
use crate::{config::Config, countdown::start_countdown};

pub fn run_daemon(cfg_base_path: PathBuf, socket_path: &str) -> std::io::Result<()> {
    if !cfg_base_path.exists() {
        panic!("Config not found. Please run: pomo init")
    }

    let cfg = Config::load(cfg_base_path).unwrap();
    let socket_path = Path::new(socket_path);
    match Lockfile::create("/tmp/pomo-daemon.lock") {
        Ok(_lock) => (),
        Err(_) => {
            panic!("Daemon already running");
        }
    }

    if socket_path.exists() {
        fs::remove_file(socket_path)?;
    }

    // channels
    let (msg_tx, msg_rx) = mpsc::channel::<Message>();

    // spawn countdown
    thread::spawn(move || start_countdown(cfg, msg_rx));

    // Unix socket listner
    let listner = UnixListener::bind(&socket_path)?;

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
    stream.shutdown(std::net::Shutdown::Write)?;
    Ok(())
}
