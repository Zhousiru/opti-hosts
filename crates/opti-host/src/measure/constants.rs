use std::{sync::LazyLock, time::Duration};

pub const GLOBALPING_API_BASE: &str = "https://api.globalping.io";
pub const GLOBALPING_MAX_TRIES: usize = 20;
pub const GLOBALPING_TRIES_INTERVAL: Duration = Duration::from_millis(500);

pub static PING_CLIENT_V4_CONFIG: LazyLock<surge_ping::Config> =
  LazyLock::new(|| surge_ping::Config {
    kind: surge_ping::ICMP::V4,
    ..Default::default()
  });
pub static PING_CLIENT_V6_CONFIG: LazyLock<surge_ping::Config> =
  LazyLock::new(|| surge_ping::Config {
    kind: surge_ping::ICMP::V6,
    ..Default::default()
  });
pub const PING_PAYLOAD: [u8; 56] = [0; 56];
pub const PING_INTERVAL: Duration = Duration::from_millis(1000);
pub const PING_TIMEOUT: Duration = Duration::from_millis(2000);
pub const PING_MAX_TRIES: u16 = 4;
pub const PING_TASK_CONCURRENT: usize = 16;
