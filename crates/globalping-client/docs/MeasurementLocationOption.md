# MeasurementLocationOption

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**continent** | Option<[**models::ContinentCode**](ContinentCode.md)> |  | [optional]
**region** | Option<[**models::RegionName**](RegionName.md)> |  | [optional]
**country** | Option<**String**> | A two-letter country code based on [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements). | [optional]
**state** | Option<**String**> | A two-letter [US state code](https://www.faa.gov/air_traffic/publications/atpubs/cnt_html/appendix_a.html). | [optional]
**city** | Option<**String**> | A city name in English. | [optional]
**asn** | Option<**i32**> | An autonomous system number (ASN). | [optional]
**network** | Option<**String**> | A network name, such as \"Google LLC\" or \"DigitalOcean, LLC\". | [optional]
**tags** | Option<**Vec<String>**> | An array of additional values to fine-tune probe selection: - Probes hosted in [AWS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-available-regions) and [Google Cloud](https://cloud.google.com/compute/docs/regions-zones#available) are automatically assigned the service region code. For example: `aws-eu-west-1` and `gcp-us-south1`. - Probes are automatically assigned `datacenter-network` and `eyeball-network` tags to distinguish between datacenter and end-user locations.  | [optional]
**magic** | Option<**String**> | Locations defined in a single string instead of the respective location properties. The API performs fuzzy matching on the `country`, `city`, `state`, `continent`, `region`, `asn` (using `AS` prefix, e.g., `AS123`), `tags`, and `network` values. Supports full names, ISO codes (where applicable), and common aliases. Multiple conditions can be combined using the `+` character, which behaves like a logical `AND`.  | [optional]
**limit** | Option<**i32**> | The maximum number of probes that should run the measurement in this location. The result count might be lower if there aren't enough probes available in this location. Mutually exclusive with the global `limit` property.  | [optional][default to 1]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


