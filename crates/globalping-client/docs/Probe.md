# Probe

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | **String** | The probe version. | 
**location** | [**models::ProbeLocation**](ProbeLocation.md) |  | 
**tags** | **Vec<String>** | An array of additional values to fine-tune probe selection: - Probes hosted in [AWS](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/using-regions-availability-zones.html#concepts-available-regions) and [Google Cloud](https://cloud.google.com/compute/docs/regions-zones#available) are automatically assigned the service region code. For example: `aws-eu-west-1` and `gcp-us-south1`. - Probes are automatically assigned `datacenter-network` and `eyeball-network` tags to distinguish between datacenter and end-user locations.  | 
**resolvers** | [**Vec<models::ProbeResolver>**](ProbeResolver.md) | An array of the default resolvers configured on the probe. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


