use std::{sync::mpsc::Receiver, thread, time::Duration};

use crate::types::{CountdownType, Message};

#[derive(Debug, Clone, Copy)]
enum CountdownState {
    Started {
        countdown_type: CountdownType,
        remaining_time: Duration,
        countdown_count: u64,
    },
    Stopped {
        countdown_type: CountdownType,
        remaining_time: Duration,
        countdown_count: u64,
    },
    Finished {
        countdown_type: CountdownType,
        countdown_count: u64,
    },
}

// Countdown state machine
impl CountdownState {
    pub fn new() -> Self {
        Self::Stopped {
            countdown_type: CountdownType::Focus,
            remaining_time: Duration::from_secs(0),
            countdown_count: 0,
        }
    }

    fn next_countdown_type(countdown_type: CountdownType) -> CountdownType {
        match countdown_type {
            CountdownType::Focus => CountdownType::Rest,
            CountdownType::Rest => CountdownType::Focus,
        }
    }

    fn next_countdown_duration(countdown_type: CountdownType, countdown_count: u64) -> Duration {
        match countdown_type {
            CountdownType::Focus => Duration::from_secs(25 * 60),
            CountdownType::Rest => match countdown_count {
                0..=3 => Duration::from_secs(5 * 60),
                _ => Duration::from_secs(15 * 60),
            },
        }
    }

    // state machine change
    pub fn next(self) -> CountdownState {
        match self {
            Self::Started {
                countdown_type,
                remaining_time,
                countdown_count,
            } => {
                if remaining_time.as_secs() > 0 {
                    Self::Started {
                        countdown_type,
                        countdown_count,
                        remaining_time: remaining_time - Duration::from_secs(1),
                    }
                } else {
                    Self::Finished {
                        countdown_type,
                        countdown_count: countdown_count + 1,
                    }
                }
            }
            Self::Stopped {
                countdown_type,
                remaining_time,
                countdown_count,
            } => Self::Stopped {
                countdown_type,
                remaining_time,
                countdown_count,
            },
            Self::Finished {
                countdown_type,
                countdown_count,
            } => Self::Finished {
                countdown_type,
                countdown_count,
            },
        }
    }

    // handles message
    pub fn handle_message(&mut self, msg: Message) -> Option<CountdownState> {
        match self {
            Self::Started {
                countdown_type,
                remaining_time,
                countdown_count,
            } => match msg {
                Message::Stop => Some(Self::Stopped {
                    countdown_type: *countdown_type,
                    remaining_time: *remaining_time,
                    countdown_count: *countdown_count,
                }),
                Message::Start => None,
            },
            Self::Stopped {
                countdown_type,
                remaining_time,
                countdown_count,
            } => match msg {
                Message::Start => match *remaining_time {
                    Duration::ZERO => Some(Self::Started {
                        remaining_time: Self::next_countdown_duration(
                            *countdown_type,
                            *countdown_count,
                        ),
                        countdown_type: *countdown_type,
                        countdown_count: *countdown_count,
                    }),
                    _ => Some(Self::Started {
                        remaining_time: *remaining_time,
                        countdown_type: *countdown_type,
                        countdown_count: *countdown_count,
                    }),
                },
                Message::Stop => None,
            },
            Self::Finished {
                countdown_type,
                countdown_count,
            } => match msg {
                Message::Start => {
                    let next_countdown_type = Self::next_countdown_type(*countdown_type);

                    Some(Self::Started {
                        remaining_time: Self::next_countdown_duration(
                            next_countdown_type,
                            *countdown_count,
                        ),
                        countdown_type: next_countdown_type,
                        countdown_count: *countdown_count,
                    })
                }
                Message::Stop => None,
            },
        }
    }
}

// countdown creates new state machine, handles messages and updates state every second
pub fn countdown(msg_rx: Receiver<Message>) {
    let mut countdown_state = CountdownState::new();

    loop {
        // read channel for new messages
        // and send them to the state machine
        if let Ok(msg) = msg_rx.try_recv() {
            countdown_state = match countdown_state.handle_message(msg) {
                Some(countdown_state) => countdown_state,
                None => countdown_state,
            }
        }

        // next state
        countdown_state = countdown_state.next();
        println!("{:?}", countdown_state);
        thread::sleep(Duration::from_secs(1));
    }
}
