use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownType {
    Focus,
    Rest,
}

impl fmt::Display for CountdownType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Rest => "Rest",
            Self::Focus => "Focus",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Start,
    Stop,
}
