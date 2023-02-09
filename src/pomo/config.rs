struct Config {
    focus_duration: u32,
    short_break_duration: u32,
    long_break_duration: u32,
}

impl Config {
    pub fn default() -> Self {
        Self {
            focus_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
        }
    }
}
