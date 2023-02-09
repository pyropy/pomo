use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum CountdownType {
    Focus,
    Rest,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Start,
    Stop,
}
