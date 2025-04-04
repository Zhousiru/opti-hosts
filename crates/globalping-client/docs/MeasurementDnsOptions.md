# MeasurementDnsOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**query** | Option<[**models::MeasurementDnsOptionsQuery**](MeasurementDnsOptions_query.md)> |  | [optional]
**resolver** | Option<[**models::MeasurementResolver**](MeasurementResolver.md)> |  | [optional]
**port** | Option<**i32**> | The port number to send the query to. | [optional][default to 53]
**protocol** | Option<**String**> | The protocol to use for the DNS query. | [optional][default to Udp]
**ip_version** | Option<**i32**> | EXPERIMENTAL: The IP version to use. Only allowed if the target is a hostname.  | [optional]
**trace** | Option<**bool**> | Toggles tracing of the delegation path from the root servers down to the target domain name.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


