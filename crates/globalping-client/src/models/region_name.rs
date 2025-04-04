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

/// RegionName : A geographic region name based on UN [Standard Country or Area Codes for Statistical Use (M49)](https://unstats.un.org/unsd/methodology/m49/). 
/// A geographic region name based on UN [Standard Country or Area Codes for Statistical Use (M49)](https://unstats.un.org/unsd/methodology/m49/). 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RegionName {
    #[serde(rename = "Northern Africa")]
    NorthernAfrica,
    #[serde(rename = "Eastern Africa")]
    EasternAfrica,
    #[serde(rename = "Middle Africa")]
    MiddleAfrica,
    #[serde(rename = "Southern Africa")]
    SouthernAfrica,
    #[serde(rename = "Western Africa")]
    WesternAfrica,
    #[serde(rename = "Caribbean")]
    Caribbean,
    #[serde(rename = "Central America")]
    CentralAmerica,
    #[serde(rename = "South America")]
    SouthAmerica,
    #[serde(rename = "Northern America")]
    NorthernAmerica,
    #[serde(rename = "Central Asia")]
    CentralAsia,
    #[serde(rename = "Eastern Asia")]
    EasternAsia,
    #[serde(rename = "South-eastern Asia")]
    SouthEasternAsia,
    #[serde(rename = "Southern Asia")]
    SouthernAsia,
    #[serde(rename = "Western Asia")]
    WesternAsia,
    #[serde(rename = "Eastern Europe")]
    EasternEurope,
    #[serde(rename = "Northern Europe")]
    NorthernEurope,
    #[serde(rename = "Southern Europe")]
    SouthernEurope,
    #[serde(rename = "Western Europe")]
    WesternEurope,
    #[serde(rename = "Australia and New Zealand")]
    AustraliaAndNewZealand,
    #[serde(rename = "Melanesia")]
    Melanesia,
    #[serde(rename = "Micronesia")]
    Micronesia,
    #[serde(rename = "Polynesia")]
    Polynesia,

}

impl std::fmt::Display for RegionName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NorthernAfrica => write!(f, "Northern Africa"),
            Self::EasternAfrica => write!(f, "Eastern Africa"),
            Self::MiddleAfrica => write!(f, "Middle Africa"),
            Self::SouthernAfrica => write!(f, "Southern Africa"),
            Self::WesternAfrica => write!(f, "Western Africa"),
            Self::Caribbean => write!(f, "Caribbean"),
            Self::CentralAmerica => write!(f, "Central America"),
            Self::SouthAmerica => write!(f, "South America"),
            Self::NorthernAmerica => write!(f, "Northern America"),
            Self::CentralAsia => write!(f, "Central Asia"),
            Self::EasternAsia => write!(f, "Eastern Asia"),
            Self::SouthEasternAsia => write!(f, "South-eastern Asia"),
            Self::SouthernAsia => write!(f, "Southern Asia"),
            Self::WesternAsia => write!(f, "Western Asia"),
            Self::EasternEurope => write!(f, "Eastern Europe"),
            Self::NorthernEurope => write!(f, "Northern Europe"),
            Self::SouthernEurope => write!(f, "Southern Europe"),
            Self::WesternEurope => write!(f, "Western Europe"),
            Self::AustraliaAndNewZealand => write!(f, "Australia and New Zealand"),
            Self::Melanesia => write!(f, "Melanesia"),
            Self::Micronesia => write!(f, "Micronesia"),
            Self::Polynesia => write!(f, "Polynesia"),
        }
    }
}

impl Default for RegionName {
    fn default() -> RegionName {
        Self::NorthernAfrica
    }
}

