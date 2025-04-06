use anyhow::{Context, Result, bail};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub async fn parse_response<T>(response: reqwest::Response) -> Result<T>
where
  T: DeserializeOwned,
{
  let status = response.status();
  let bytes = response
    .bytes()
    .await
    .context("Failed to read response body")?;

  if !status.is_success() {
    if let Ok(value) = serde_json::from_slice::<Value>(&bytes) {
      bail!("Request failed with status {}:\n {:#?}", status, value);
    } else {
      bail!("Request failed with status {}", status);
    }
  }

  let parsed: T = serde_json::from_slice(&bytes).context("Failed to deserialize response body")?;

  Ok(parsed)
}
