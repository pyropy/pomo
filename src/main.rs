use clap::{Parser, Subcommand};
use pomo::daemon::{run_daemon, send_message};
use pomo::types::Message;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// starts pomo daemon
    Daemon,
    /// starts pomo timer
    Start,
    /// stops pomo timer
    Stop,
}

fn main() {
    // 1. Load config from config folder if folder exists, or init config
    // 2. match cli command to corrent function
    // daemon   -  Start daemon listening at unix socket
    // start    -  Send start message to unix socket
    // stop     -  Send start message to unix socket
    // restart  -  Send restart message to unix socket
    // kill     -  Send kill message to unix socket
    // info     -  Load stats from embedded db directly
    let args = Cli::parse();
    let socket_path = "/tmp/pomo.sock";

    match &args.command {
        Some(Commands::Daemon) => run_daemon(socket_path).unwrap(),
        Some(Commands::Start) => send_message(socket_path, Message::Start).unwrap(),
        Some(Commands::Stop) => send_message(socket_path, Message::Stop).unwrap(),
        None => println!("No command given"),
    }
}
