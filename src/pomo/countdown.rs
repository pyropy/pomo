use std::{sync::mpsc::Receiver, thread, time::Duration};

use crate::{
    config::Config,
    types::{CountdownType, Message},
};

#[derive(Debug, Clone, Copy)]
enum CountdownState {
    Started {
        config: Config,
        countdown_type: CountdownType,
        remaining_time: Duration,
        focus_countdown_count: u64,
    },
    Stopped {
        config: Config,
        countdown_type: CountdownType,
        remaining_time: Duration,
        focus_countdown_count: u64,
    },
    Finished {
        config: Config,
        countdown_type: CountdownType,
        focus_countdown_count: u64,
    },
}

// Countdown state machine
impl CountdownState {
    pub fn new(config: Config) -> Self {
        Self::Stopped {
            config,
            countdown_type: CountdownType::Focus,
            remaining_time: Duration::from_secs(0),
            focus_countdown_count: 1,
        }
    }

    fn next_countdown_type(prev_countdown_type: CountdownType) -> CountdownType {
        match prev_countdown_type {
            CountdownType::Focus => CountdownType::Rest,
            CountdownType::Rest => CountdownType::Focus,
        }
    }

    // returns duration of countdown based on countdown type and count
    fn countdown_duration(
        cfg: Config,
        countdown_type: CountdownType,
        focus_countdown_count: u64,
    ) -> Duration {
        match countdown_type {
            CountdownType::Focus => Duration::from_secs(cfg.focus_duration * 60),
            CountdownType::Rest => match focus_countdown_count % (cfg.long_break_after + 1) {
                // +1 because we start with 1
                0 => Duration::from_secs(cfg.long_break_after * 60),
                _ => Duration::from_secs(cfg.short_break_duration * 60),
            },
        }
    }

    // state machine change
    pub fn next(self) -> CountdownState {
        match self {
            Self::Started {
                config,
                countdown_type,
                remaining_time,
                focus_countdown_count,
            } => {
                if remaining_time.as_secs() > 0 {
                    Self::Started {
                        config,
                        countdown_type,
                        focus_countdown_count,
                        remaining_time: remaining_time - Duration::from_secs(1),
                    }
                } else {
                    let focus_countdown_count = match countdown_type {
                        CountdownType::Focus => focus_countdown_count + 1,
                        CountdownType::Rest => focus_countdown_count,
                    };

                    Self::Finished {
                        config,
                        countdown_type,
                        focus_countdown_count,
                    }
                }
            }
            Self::Stopped {
                config,
                countdown_type,
                remaining_time,
                focus_countdown_count,
            } => Self::Stopped {
                config,
                countdown_type,
                remaining_time,
                focus_countdown_count,
            },
            Self::Finished {
                config,
                countdown_type,
                focus_countdown_count,
            } => Self::Finished {
                config,
                countdown_type,
                focus_countdown_count,
            },
        }
    }

    // handles message
    pub fn handle_message(self, msg: Message) -> Option<CountdownState> {
        match self {
            Self::Started {
                config,
                countdown_type,
                remaining_time,
                focus_countdown_count,
            } => match msg {
                Message::Stop => Some(Self::Stopped {
                    config,
                    countdown_type,
                    remaining_time,
                    focus_countdown_count,
                }),
                Message::Start => None,
            },
            Self::Stopped {
                config,
                countdown_type,
                remaining_time,
                focus_countdown_count,
            } => match msg {
                Message::Start => match remaining_time {
                    Duration::ZERO => Some(Self::Started {
                        config,
                        remaining_time: Self::countdown_duration(
                            config,
                            countdown_type,
                            focus_countdown_count,
                        ),
                        countdown_type,
                        focus_countdown_count,
                    }),
                    _ => Some(Self::Started {
                        config,
                        remaining_time,
                        countdown_type,
                        focus_countdown_count,
                    }),
                },
                Message::Stop => None,
            },
            Self::Finished {
                config,
                countdown_type,
                focus_countdown_count,
            } => match msg {
                Message::Start => {
                    let next_countdown_type = Self::next_countdown_type(countdown_type);

                    Some(Self::Started {
                        config,
                        remaining_time: Self::countdown_duration(
                            config,
                            next_countdown_type,
                            focus_countdown_count,
                        ),
                        countdown_type: next_countdown_type,
                        focus_countdown_count,
                    })
                }
                Message::Stop => None,
            },
        }
    }
}

// start_countdown creates new state machine, handles messages and updates state every second
pub fn start_countdown(config: Config, msg_rx: Receiver<Message>) {
    let mut countdown_state = CountdownState::new(config);

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
        eprintln!("{:?}", countdown_state);
        thread::sleep(Duration::from_secs(1));
    }
}
