# FinishedDnsTestResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**models::FinishedTestStatus**](FinishedTestStatus.md) |  | 
**raw_output** | **String** | The raw output of the test. Can be presented to users but is not meant to be parsed by clients. Please use the individual values provided in other fields for automated processing.  | 
**status_code** | **i32** | The DNS [response code](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#table-dns-parameters-6). | 
**status_code_name** | **String** | The DNS [response code name](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#table-dns-parameters-6). | 
**resolver** | **String** | The hostname or IP of the resolver that answered the query. | 
**answers** | [**Vec<models::DnsTestAnswer>**](DnsTestAnswer.md) | An array of the received resource records. | 
**timings** | [**models::DnsTestHopResultTimings**](DnsTestHopResult_timings.md) |  | 
**hops** | [**Vec<models::DnsTestHopResult>**](DnsTestHopResult.md) | An array containing details about each hop. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


