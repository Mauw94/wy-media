use std::time::Duration;

pub struct Config {
    pub refresh_rate: Duration,
    pub tick_gap: Duration,
}

impl Config {
    pub fn default() -> Self {
        Self {
            refresh_rate: Duration::from_millis(50),
            tick_gap: Duration::from_millis(100),
        }
    }
}
