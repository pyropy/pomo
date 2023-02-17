use bincode;
use std::{fs::File, time::Duration};

use crate::countdown::CountdownState;

pub fn current_state() {
    let state_file_path = "/tmp/pomo.state";
    if let Ok(reader) = File::open(state_file_path) {
        let result: Result<CountdownState, Box<bincode::ErrorKind>> =
            bincode::deserialize_from(&reader);

        match result {
            Ok(state) => print_state(state),
            Err(e) => panic!("Error while reading state {:?}", e),
        }
        return;
    }
    println!("Failed to read state from disk")
}

fn print_state(state: CountdownState) {
    match state {
        CountdownState::Started {
            countdown_type,
            remaining_time,
            ..
        } => {
            let (hrs, min, sec) = format_duration(remaining_time);
            println!("🍅 {} {}:{}:{}", countdown_type, hrs, min, sec);
        }
        CountdownState::Stopped {
            countdown_type,
            remaining_time,
            ..
        } => {
            let (hrs, min, sec) = format_duration(remaining_time);
            println!("⏸  {} {}:{}:{}", countdown_type, hrs, min, sec);
        }
        CountdownState::Finished { .. } => println!("🏁 Finished."),
    }
}

fn format_duration(dur: Duration) -> (u64, u64, u64) {
    let hours = (dur.as_secs() / 60) / 60;
    let minutes = (dur.as_secs() / 60) % 60;
    let seconds = dur.as_secs() % 60;

    (hours, minutes, seconds)
}
