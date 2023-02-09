pub struct Config {
    pub focus_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub long_break_after: u64,
}

impl Config {
    pub fn default() -> Self {
        Self {
            focus_duration: 25,
            short_break_duration: 5,
            long_break_duration: 25,
            long_break_after: 4,
        }
    }
}
