/*
 * Globalping API
 *
 * The Globalping API allows you to monitor, debug, and benchmark your internet infrastructure using a globally distributed network of probes.  The API is public, free to use, and doesn't require authentication. However, it implements rate limits to ensure fair usage and reliability, and some of the limits are higher for authenticated users. Sign up on the [Globalping Dashboard](https://dash.globalping.io/) to enjoy the higher limits.  Root endpoint: https://api.globalping.io  ## Limits and credits  | Operation | Unauthenticated user | Authenticated user | |---|---|---| | **Measurements** ||| | Create a measurement | 250 tests/hour | 500 tests/hour* | | Get a measurement by ID | 2 requests/second/measurement | 2 requests/second/measurement | | **Probes** ||| | List probes currently online | no limit | no limit | | **Limits** ||| | Get current rate limits | no limit | no limit |  \\*Additional measurements may be created by spending credits. Each test above the limit costs one credit. Learn more about credits on the [Globalping website](https://globalping.io/credits).  ## Client guidelines  If you're implementing an application that interacts with the API, please refer to the \"Client guidelines\" section in the description of each endpoint. This way, you can provide the best UX and reduce the likelihood of your app breaking in the future.  ### General guidelines for non-browser-based apps:  - Set a `User-Agent` header. We recommend that you follow the format and approach [used here](https://github.com/jsdelivr/data.jsdelivr.com/blob/60c5154d26c403ba9dd403a8ddc5e42a31931f0d/config/default.js#L9). - Set an `Accept-Encoding` header with a value of either `br` (preferred) or `gzip`, depending on what your client can support. Compression has a significant impact on the response size. - Implement ETag-based client-side caching using the `ETag`/`If-None-Match` headers when requesting the measurement status. 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: d@globalping.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ProbeLocation : The probe location information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProbeLocation {
    #[serde(rename = "continent")]
    pub continent: models::ContinentCode,
    #[serde(rename = "region")]
    pub region: models::RegionName,
    /// A two-letter country code based on [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements).
    #[serde(rename = "country")]
    pub country: String,
    /// A two-letter [US state code](https://www.faa.gov/air_traffic/publications/atpubs/cnt_html/appendix_a.html).
    #[serde(rename = "state", deserialize_with = "Option::deserialize")]
    pub state: Option<String>,
    /// A city name in English.
    #[serde(rename = "city")]
    pub city: String,
    /// An autonomous system number (ASN).
    #[serde(rename = "asn")]
    pub asn: i32,
    /// A network name, such as \"Google LLC\" or \"DigitalOcean, LLC\".
    #[serde(rename = "network")]
    pub network: String,
    /// The latitude of probe location.
    #[serde(rename = "latitude")]
    pub latitude: f64,
    /// The longitude of probe location.
    #[serde(rename = "longitude")]
    pub longitude: f64,
}

impl ProbeLocation {
    /// The probe location information.
    pub fn new(continent: models::ContinentCode, region: models::RegionName, country: String, state: Option<String>, city: String, asn: i32, network: String, latitude: f64, longitude: f64) -> ProbeLocation {
        ProbeLocation {
            continent,
            region,
            country,
            state,
            city,
            asn,
            network,
            latitude,
            longitude,
        }
    }
}

