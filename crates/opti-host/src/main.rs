use anyhow::Result;
use measure::{globalping::measure_dig, ping::batch_measure_ping};
use parser::parse_hosts;
use tokio::task::JoinSet;

mod measure;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
  let result = parse_hosts(
    r#"192.168.1.1 router.home
127.0.0.1 localhost
# OPTI-HOSTS example.com [Beijing, HK * 2, AS6939]
# OPTI-HOSTS rua.sh [Alibaba * 2]
127.0.0.2 notlocalhost
# invalid below
# OPTI-HOSTS rua.sh [Tencent * 0]"#,
  );

  let mut join_set = JoinSet::new();

  for item in result {
    join_set.spawn(async move {
      let ips = measure_dig(&item.domain, item.location_params)
        .await
        .unwrap();
      println!("DOMAIN: {}, IPS: {:?}", item.domain, ips);

      let result = batch_measure_ping(ips).await;

      println!("LATENCY FOR {}: {:?}", item.domain, result)
    });
  }

  join_set.join_all().await;

  Ok(())
}
