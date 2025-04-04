# FinishedPingTestResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**models::FinishedTestStatus**](FinishedTestStatus.md) |  | 
**raw_output** | **String** | The raw output of the test. Can be presented to users but is not meant to be parsed by clients. Please use the individual values provided in other fields for automated processing.  | 
**resolved_address** | Option<**String**> | The resolved IP address of the `target`. | 
**resolved_hostname** | Option<**String**> | The resolved hostname of the `target`. | 
**stats** | [**models::FinishedPingTestResultAllOfStats**](FinishedPingTestResult_allOf_stats.md) |  | 
**timings** | [**Vec<models::FinishedPingTestResultAllOfTimings>**](FinishedPingTestResult_allOf_timings.md) | An array containing details for each packet. All times are in milliseconds.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


