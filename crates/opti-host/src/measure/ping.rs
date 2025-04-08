use std::{net::IpAddr, sync::LazyLock};

use anyhow::Context;
use futures::{StreamExt, stream};
use rand::random;
use surge_ping::{Client, PingIdentifier, PingSequence};
use tokio::time;

use super::constants::{
  PING_CLIENT_V4_CONFIG, PING_CLIENT_V6_CONFIG, PING_INTERVAL, PING_MAX_TRIES, PING_PAYLOAD,
  PING_TASK_CONCURRENT, PING_TIMEOUT,
};

static CLIENT_V4: LazyLock<Client> = LazyLock::new(|| {
  Client::new(&PING_CLIENT_V4_CONFIG)
    .context("Failed to create IPv4 ping client")
    .unwrap()
});

static CLIENT_V6: LazyLock<Client> = LazyLock::new(|| {
  Client::new(&PING_CLIENT_V6_CONFIG)
    .context("Failed to create IPv6 ping client")
    .unwrap()
});

pub async fn measure_ping(ip: String) -> Option<f32> {
  let ip_addr = ip.parse::<IpAddr>().context("Failed to parse IP").unwrap();
  let result = match ip_addr {
    IpAddr::V4(ip_addr) => ping_ip(IpAddr::V4(ip_addr), &CLIENT_V4).await,
    IpAddr::V6(ip_addr) => ping_ip(IpAddr::V6(ip_addr), &CLIENT_V6).await,
  };
  result
}

pub async fn ping_ip(ip_addr: IpAddr, client: &LazyLock<Client>) -> Option<f32> {
  let mut pinger = client.pinger(ip_addr, PingIdentifier(random())).await;
  pinger.timeout(PING_TIMEOUT);

  let mut latency_sum: usize = 0;
  let mut success_count: usize = 0;

  let mut interval = time::interval(PING_INTERVAL);

  for id in 0..PING_MAX_TRIES {
    interval.tick().await;

    match pinger.ping(PingSequence(id), &PING_PAYLOAD).await {
      Ok((_, dur)) => {
        latency_sum += dur.as_millis() as usize;
        success_count += 1
      }
      Err(_) => (),
    };
  }

  if success_count == 0 {
    return None;
  }
  return Some(latency_sum as f32 / success_count as f32);
}

pub async fn batch_measure_ping(ips: Vec<String>) -> Vec<Option<f32>> {
  let futures = ips.into_iter().map(|ip| measure_ping(ip));

  stream::iter(futures)
    .buffered(PING_TASK_CONCURRENT)
    .collect()
    .await
}
