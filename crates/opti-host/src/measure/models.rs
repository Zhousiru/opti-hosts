use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateMeasurementRequest {
  pub r#type: String,
  pub target: String,
  pub locations: Vec<LocationOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocationOption {
  pub magic: String,
  pub limit: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateMeasurementResponse {
  pub id: String,
  pub probes_count: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DnsMeasurementResponse {
  pub status: String,
  pub results: Vec<DnsResultItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DnsResultItem {
  pub result: ResultDetail,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResultDetail {
  #[serde(default)]
  pub answers: Vec<DnsAnswerItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DnsAnswerItem {
  pub name: String,
  pub r#type: String,
  pub ttl: i32,
  pub class: String,
  pub value: String,
}
