/*
 * Globalping API
 *
 * The Globalping API allows you to monitor, debug, and benchmark your internet infrastructure using a globally distributed network of probes.  The API is public, free to use, and doesn't require authentication. However, it implements rate limits to ensure fair usage and reliability, and some of the limits are higher for authenticated users. Sign up on the [Globalping Dashboard](https://dash.globalping.io/) to enjoy the higher limits.  Root endpoint: https://api.globalping.io  ## Limits and credits  | Operation | Unauthenticated user | Authenticated user | |---|---|---| | **Measurements** ||| | Create a measurement | 250 tests/hour | 500 tests/hour* | | Get a measurement by ID | 2 requests/second/measurement | 2 requests/second/measurement | | **Probes** ||| | List probes currently online | no limit | no limit | | **Limits** ||| | Get current rate limits | no limit | no limit |  \\*Additional measurements may be created by spending credits. Each test above the limit costs one credit. Learn more about credits on the [Globalping website](https://globalping.io/credits).  ## Client guidelines  If you're implementing an application that interacts with the API, please refer to the \"Client guidelines\" section in the description of each endpoint. This way, you can provide the best UX and reduce the likelihood of your app breaking in the future.  ### General guidelines for non-browser-based apps:  - Set a `User-Agent` header. We recommend that you follow the format and approach [used here](https://github.com/jsdelivr/data.jsdelivr.com/blob/60c5154d26c403ba9dd403a8ddc5e42a31931f0d/config/default.js#L9). - Set an `Accept-Encoding` header with a value of either `br` (preferred) or `gzip`, depending on what your client can support. Compression has a significant impact on the response size. - Implement ETag-based client-side caching using the `ETag`/`If-None-Match` headers when requesting the measurement status. 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: d@globalping.io
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`create_measurement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMeasurementError {
    Status400(models::InlineObject),
    Status422(models::InlineObject1),
    Status429(models::InlineObject1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_measurement`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMeasurementError {
    Status404(models::InlineObject1),
    Status429(models::InlineObject1),
    UnknownValue(serde_json::Value),
}


/// Creates a new measurement with parameters set in the request body. The measurement runs asynchronously and you can retrieve its current state at the URL returned in the `Location` header.  ### Client guidelines  - If the application is running in interactive mode, set the `inProgressUpdates` option to `true` to have the API   return partial results as soon as they are available. This allows the user to see the measurement progress in real time.   - If the application is interactive by default but also implements a \"CI\" mode for scripting, do not set the flag in the CI mode. - To perform multiple measurements using exactly the same probes, create a single measurement first, then pass its `id` in the `locations` field for the other measurements. - When you receive a `429` response, inform the user about their current rate limit status based on the response headers. Depending on the exact situation and on what your application supports, you may also suggest:   - Signing in or using an access token.   - Learning more about how to get additional credits at https://globalping.io/credits.   - Repeating the measurement with fewer probes. 
pub async fn create_measurement(configuration: &configuration::Configuration, measurement_request: Option<models::MeasurementRequest>) -> Result<models::CreateMeasurementResponse, Error<CreateMeasurementError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_measurement_request = measurement_request;

    let uri_str = format!("{}/v1/measurements", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_measurement_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CreateMeasurementResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CreateMeasurementResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CreateMeasurementError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns the status and results of an existing measurement. Measurements are typically available for up to 7 days after creation.  > **Tip**: A link to this endpoint is returned in the `Location` response header when creating the measurement.  ### Client guidelines  As it can take a few seconds for a measurement to complete, you should use the following process for retrieving the results:   1. Request the measurement to retrieve its status.   2. If the status is `in-progress`, wait 500 milliseconds and start again at step 1. Note that it's important to wait 500 ms *after* receiving the response rather than using an \"every 500ms\" interval as for large measurements, the request itself may take a few hundred milliseconds to complete.   3. If the status is anything **other** than `in-progress`, stop. The measurement is no longer running, and its results are final.  > **Important**: Do not query the results of a single measurement more often than every 500 milliseconds. Sending more than two requests per second may trigger a rate limit and prevent you from accessing the results for a few seconds. 
pub async fn get_measurement(configuration: &configuration::Configuration, id: &str) -> Result<models::InlineObject2, Error<GetMeasurementError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/v1/measurements/{id}", configuration.base_path, id=crate::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::InlineObject2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::InlineObject2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetMeasurementError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

