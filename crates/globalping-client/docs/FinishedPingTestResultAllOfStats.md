# FinishedPingTestResultAllOfStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min** | [**models::StatsRttMinNullable**](StatsRttMinNullable.md) |  | 
**avg** | [**models::StatsRttAvgNullable**](StatsRttAvgNullable.md) |  | 
**max** | [**models::StatsRttMaxNullable**](StatsRttMaxNullable.md) |  | 
**total** | **i32** | The number of packets sent. | 
**rcv** | **i32** | The number of received packets. | 
**drop** | **i32** | The number of dropped packets (`total` - `rcv`). | 
**loss** | **f64** | The percentage of dropped packets. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


