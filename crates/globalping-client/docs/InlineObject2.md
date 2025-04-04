# InlineObject2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The measurement ID. | 
**r#type** | [**models::MeasurementType**](MeasurementType.md) |  | 
**target** | **String** | A publicly reachable measurement target. Typically a hostname, an IPv4 address, or IPv6 address, depending on the measurement `type`. Support for IPv6 targets is currently considered experimental.  | 
**status** | [**models::MeasurementStatus**](MeasurementStatus.md) |  | 
**created_at** | **String** | The date and time when the measurement was created. | 
**updated_at** | **String** | The date and time when the measurement was last updated. | 
**probes_count** | **i32** | The actual number of probes that performed the measurement tests. Smaller or equal to `limit`, depending on probe availability.  | 
**locations** | Option<[**Vec<models::MeasurementLocationOption>**](MeasurementLocationOption.md)> | The locations you specified when creating the measurement. If you passed in an `id` of a previous measurement, the value will be filled in from that measurement.  | [optional]
**limit** | Option<**i32**> |  | [optional]
**measurement_options** | Option<[**models::MeasurementResponseMeasurementOptions**](MeasurementResponse_measurementOptions.md)> |  | [optional]
**results** | [**Vec<models::MeasurementResultItem>**](MeasurementResultItem.md) | An array containing the measurement results. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


