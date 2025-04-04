# MeasurementResultItemProbe

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**continent** | [**models::ContinentCode**](ContinentCode.md) |  | 
**region** | [**models::RegionName**](RegionName.md) |  | 
**country** | **String** | A two-letter country code based on [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements). | 
**state** | **String** | A two-letter [US state code](https://www.faa.gov/air_traffic/publications/atpubs/cnt_html/appendix_a.html). | 
**city** | **String** | A city name in English. | 
**asn** | **i32** | An autonomous system number (ASN). | 
**network** | **String** | A network name, such as \"Google LLC\" or \"DigitalOcean, LLC\". | 
**latitude** | **f64** | The latitude of probe location. | 
**longitude** | **f64** | The longitude of probe location. | 
**tags** | **Vec<String>** | An array of additional values to fine-tune probe selection: - Probes hosted in [AWS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-available-regions) and [Google Cloud](https://cloud.google.com/compute/docs/regions-zones#available) are automatically assigned the service region code. For example: `aws-eu-west-1` and `gcp-us-south1`. - Probes are automatically assigned `datacenter-network` and `eyeball-network` tags to distinguish between datacenter and end-user locations.  | 
**resolvers** | [**Vec<models::ProbeResolver>**](ProbeResolver.md) | An array of the default resolvers configured on the probe. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


