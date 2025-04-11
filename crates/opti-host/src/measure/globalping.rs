use std::{collections::HashSet, sync::LazyLock};

use anyhow::{Context, Result, bail};
use tokio::time::sleep;

use super::constants::{GLOBALPING_MAX_TRIES, GLOBALPING_TRIES_INTERVAL};
use crate::{
  measure::{
    constants::GLOBALPING_API_BASE,
    models::{
      CreateMeasurementRequest, CreateMeasurementResponse, DnsMeasurementResponse, LocationOption,
    },
    utils::parse_response,
  },
  parser::LocationParam,
};

static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

pub async fn measure_dig(domain: &str, location_params: &[LocationParam]) -> Result<Vec<String>> {
  let location_option = location_params
    .iter()
    .map(|item| LocationOption {
      magic: item.location.to_owned(),
      limit: (item.limit as i32),
    })
    .collect();

  let resp = CLIENT
    .post(format!("{}/v1/measurements", GLOBALPING_API_BASE))
    .json(&CreateMeasurementRequest {
      r#type: "dns".into(),
      target: domain.into(),
      locations: location_option,
    })
    .send()
    .await
    .context("Failed to send create globalping dns measurement request")?;

  let resp = parse_response::<CreateMeasurementResponse>(resp)
    .await
    .context("Failed to create globalping dns measurement")?;

  let measurement_id = resp.id;

  let final_resp: DnsMeasurementResponse;
  let mut tries: usize = 0;

  loop {
    tries += 1;
    if tries > GLOBALPING_MAX_TRIES {
      bail!("Max get measurement tries limit exceeded");
    }

    let resp = CLIENT
      .get(format!(
        "{}/v1/measurements/{}",
        GLOBALPING_API_BASE, measurement_id
      ))
      .send()
      .await
      .context("Failed to send get globalping measurement request")?;

    let resp = parse_response::<DnsMeasurementResponse>(resp).await;

    let resp = match resp {
      Err(_) => {
        sleep(GLOBALPING_TRIES_INTERVAL).await;
        continue;
      }
      Ok(resp) => resp,
    };

    if resp.status != "finished" {
      sleep(GLOBALPING_TRIES_INTERVAL).await;
      continue;
    }

    final_resp = resp;

    break;
  }

  let mut ips: HashSet<String> = HashSet::new();

  for detail in final_resp.results.into_iter().map(|v| v.result) {
    for answer in detail.answers {
      if answer.r#type != "A" || answer.class != "IN" {
        continue;
      };
      ips.insert(answer.value);
    }
  }

  Ok(ips.into_iter().collect())
}
