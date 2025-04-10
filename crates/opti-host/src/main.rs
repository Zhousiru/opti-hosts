use std::f32;

use anyhow::{Context, Result};
use clap::Parser;
use futures::future::join_all;
use measure::{globalping::measure_dig, ping::batch_measure_ping};
use parser::parse_hosts;
use tokio::fs;

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
    "Failed to read the hosts file. Please make sure you have sufficient permissions to read the hosts file",
  ).unwrap()).context("Failed to read the hosts file. Please make sure the file is encoded in UTF-8").unwrap();

  let directive_metas = parse_hosts(&content);

  println!(
    "{} directive(s) parsed from `{}`",
    directive_metas.len(),
    cli_args.file
  );

  println!("Retrieving IP(s) from Globalping...");

  let directive_ips = join_all(directive_metas.iter().map(|meta| async move {
    let ips = measure_dig(&meta.domain, &meta.location_params)
      .await
      .unwrap();

    println!(
      "{} IP(s) of `{}` retrieved from [{}]",
      ips.len(),
      meta.domain,
      meta
        .location_params
        .iter()
        .map(|x| { x.to_string() })
        .collect::<Vec<_>>()
        .join(", ")
    );
    for ip in &ips {
      println!(" - {}", ip)
    }

    ips
  }))
  .await;

  println!("Testing latency...");

  let mut directive_selected: Vec<Option<String>> = vec![None; directive_ips.len()];

  for (index, ips) in directive_ips.iter().enumerate() {
    let latency = batch_measure_ping(ips.clone()).await;
    let min_index = latency
      .iter()
      .enumerate()
      .min_by(|a, b| {
        let a_val = a.1.unwrap_or(f32::INFINITY);
        let b_val = b.1.unwrap_or(f32::INFINITY);
        a_val
          .partial_cmp(&b_val)
          .unwrap_or(std::cmp::Ordering::Equal)
      })
      .map(|x| x.0);

    if min_index.is_none() {
      continue;
    }
    let min_index = min_index.unwrap();

    let selected_index = if latency[min_index].is_none() {
      None
    } else {
      directive_selected[index] = Some(ips[min_index].clone());
      Some(min_index)
    };

    println!("Latency of `{}`", directive_metas[index].domain);
    for (index, item) in ips.iter().zip(latency.iter()).enumerate() {
      if Some(index) == selected_index {
        println!(
          " - {:<15} {:<10} (selected)",
          item.0,
          item.1.map_or("timeout".into(), |x| format!("{:.2} ms", x))
        );
      } else {
        println!(
          " - {:<15} {:<10}",
          item.0,
          item.1.map_or("timeout".into(), |x| format!("{:.2} ms", x))
        );
      }
    }
  }

  println!("Generating hosts file...");

  let mut new_content_lines: Vec<String> = content.lines().map(|x| x.to_owned()).collect();

  for (meta, selected) in directive_metas.iter().zip(directive_selected.iter()).rev() {
    let line = match selected {
      Some(ip) => format!("{} {}", ip, meta.domain),
      None => "# Failed to determine optimal IP".to_string(),
    };

    let next_line_index = meta.line_index + 1;
    let should_replace = if next_line_index < new_content_lines.len() {
      let next_line = &new_content_lines[next_line_index];
      if next_line.starts_with("#") {
        false
      } else {
        let parts: Vec<&str> = next_line.split_whitespace().collect();
        parts.len() >= 2 && parts[1] == meta.domain
      }
    } else {
      false
    };

    if should_replace {
      new_content_lines[next_line_index] = line;
    } else {
      new_content_lines.insert(meta.line_index + 1, line);
    }
  }

  let new_content = new_content_lines.join("\n");

  println!("{:-^42}", " Generated hosts ");
  println!("{}", new_content);
  println!("{:-^42}", " Generated hosts ");

  if !cli_args.dry_run {
    println!("Writing to hosts file `{}`", cli_args.file);
    fs::write(cli_args.file, new_content).await.context("Failed to write the hosts file. Please make sure you have sufficient permissions to write the hosts file").unwrap();
  }

  Ok(())
}
