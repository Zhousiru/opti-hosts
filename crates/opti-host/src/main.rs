use anyhow::{Context, Result};
use clap::Parser;
use measure::globalping::measure_dig;
use parser::parse_hosts;
use tokio::{fs, task::JoinSet};

mod measure;
mod parser;

#[derive(clap::Parser, Debug)]
#[command(name = "opti-host")]
#[command(about = "Resolve domains by latency, not CDN zones", long_about = None)]
struct CliArgs {
  /// Preview outputs without making any changes to hosts file
  #[arg(long, default_value_t = false)]
  dry_run: bool,

  /// Hosts file path
  #[arg(long, default_value = "/etc/hosts")]
  file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli_args = CliArgs::parse();

  let content = String::from_utf8(fs::read(&cli_args.file).await.context(
    "Failed to read the hosts file. Please make sure you have sufficient permissions to edit the hosts file.",
  ).unwrap()).context("Failed to read the hosts file. Please make sure the file is encoded in UTF-8").unwrap();

  let directive_metas = parse_hosts(&content);

  println!(
    "{} directive(s) parsed from `{}`",
    directive_metas.len(),
    cli_args.file
  );

  println!("Retrieving IP(s) from Globalping...");

  let mut join_set = JoinSet::new();

  for meta in directive_metas {
    join_set.spawn(async move {
      let ips = measure_dig(&meta.domain, &meta.location_params)
        .await
        .unwrap();

      println!(
        "{} IP(s) of `{}` retrieved from [{}]:",
        ips.len(),
        meta.domain,
        meta
          .location_params
          .iter()
          .map(|x| { x.to_string() })
          .collect::<Vec<_>>()
          .join(", ")
      );
      for ip in ips {
        println!(" - {}", ip)
      }
    });
  }

  join_set.join_all().await;

  println!("Testing latency...");

  Ok(())
}
