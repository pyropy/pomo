use clap::{Parser, Subcommand};
use dirs::home_dir;
use pomo::config::Config;
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
    /// Init default config
    Init,
    /// starts pomo daemon
    Daemon,
    /// starts pomo timer
    Start,
    /// stops pomo timer
    Stop,
}

fn main() {
    let args = Cli::parse();
    let socket_path = "/tmp/pomo.sock";
    let home_dir = home_dir().unwrap();
    let default_config_base_path = home_dir.join(".config/pomo");

    match &args.command {
        Some(Commands::Init) => Config::init(default_config_base_path).unwrap(),
        Some(Commands::Daemon) => run_daemon(default_config_base_path, socket_path).unwrap(),
        Some(Commands::Start) => send_message(socket_path, Message::Start).unwrap(),
        Some(Commands::Stop) => send_message(socket_path, Message::Stop).unwrap(),
        None => println!("No command given"),
    }
}
