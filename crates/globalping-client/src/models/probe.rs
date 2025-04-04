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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Probe {
    /// The probe version.
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "location")]
    pub location: Box<models::ProbeLocation>,
    /// An array of additional values to fine-tune probe selection: - Probes hosted in [AWS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-available-regions) and [Google Cloud](https://cloud.google.com/compute/docs/regions-zones#available) are automatically assigned the service region code. For example: `aws-eu-west-1` and `gcp-us-south1`. - Probes are automatically assigned `datacenter-network` and `eyeball-network` tags to distinguish between datacenter and end-user locations. 
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    /// An array of the default resolvers configured on the probe.
    #[serde(rename = "resolvers")]
    pub resolvers: Vec<models::ProbeResolver>,
}

impl Probe {
    pub fn new(version: String, location: models::ProbeLocation, tags: Vec<String>, resolvers: Vec<models::ProbeResolver>) -> Probe {
        Probe {
            version,
            location: Box::new(location),
            tags,
            resolvers,
        }
    }
}

