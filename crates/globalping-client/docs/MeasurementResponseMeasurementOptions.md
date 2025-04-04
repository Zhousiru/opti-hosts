# MeasurementResponseMeasurementOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**packets** | Option<**i32**> | The number of packets to send to each hop. | [optional][default to 3]
**ip_version** | Option<**i32**> | EXPERIMENTAL: The IP version to use. Only allowed if the target is a hostname.  | [optional]
**port** | Option<**i32**> | The port number to use. | [optional][default to 80]
**protocol** | Option<**String**> | The transport protocol to use. | [optional][default to Https]
**query** | Option<[**models::MeasurementDnsOptionsQuery**](MeasurementDnsOptions_query.md)> |  | [optional]
**resolver** | Option<[**models::MeasurementResolver**](MeasurementResolver.md)> |  | [optional]
**trace** | Option<**bool**> | Toggles tracing of the delegation path from the root servers down to the target domain name.  | [optional][default to false]
**request** | Option<[**models::MeasurementHttpOptionsRequest**](MeasurementHttpOptions_request.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


