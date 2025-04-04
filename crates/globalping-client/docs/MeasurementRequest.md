# MeasurementRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::MeasurementType**](MeasurementType.md) |  | 
**target** | **String** | A publicly reachable measurement target. Typically a hostname, an IPv4 address, or IPv6 address, depending on the measurement `type`. Support for IPv6 targets is currently considered experimental.  | 
**in_progress_updates** | Option<**bool**> | Indicates whether you want to get partial results while the measurement is still running: - If `true`, partial results are returned as soon as they are available, and you can present them to the user in real time. Note that only the first 5 tests from the `results` array will update in real time. - If `false`, the result of each test is updated only after the test finishes.  | [optional][default to false]
**locations** | Option<[**models::MeasurementLocations**](MeasurementLocations.md)> |  | [optional]
**limit** | Option<**i32**> | The maximum number of probes that should run the measurement. The result count might be lower if there aren't enough probes available in the specified locations. Mutually exclusive with the `limit` property that can be set for individual locations.  | [optional][default to 1]
**measurement_options** | Option<[**models::MeasurementOptions**](MeasurementOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


