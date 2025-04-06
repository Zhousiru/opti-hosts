use std::time::Duration;

pub const API_BASE: &str = "https://api.globalping.io";
pub const MAX_TRIES: usize = 20;
pub const TRIES_INTERVAL: Duration = Duration::from_millis(500);
