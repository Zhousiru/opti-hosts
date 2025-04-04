# FinishedHttpTestResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**status** | [**models::FinishedTestStatus**](FinishedTestStatus.md) |  | 
**raw_output** | **String** | The raw output of the test. Can be presented to users but is not meant to be parsed by clients. Please use the individual values provided in other fields for automated processing.  | 
**raw_headers** | **String** | The raw HTTP response headers. | 
**raw_body** | Option<**String**> | The raw HTTP response body or `null` if there was no body in response. Note that only the first 10 kb are returned.  | 
**truncated** | **bool** | Indicates whether the `rawBody` value was truncated due to being too big.  | 
**headers** | [**std::collections::HashMap<String, models::FinishedHttpTestResultAllOfHeaders>**](FinishedHttpTestResult_allOf_headers.md) | The HTTP response headers. The value may be an array of strings for headers with multiple values, e.g., `Set-Cookie`. | 
**status_code** | **i32** | The HTTP response status code. | 
**status_code_name** | **String** | The HTTP response status code name. | 
**resolved_address** | Option<**String**> | The resolved IP address of the `target`. | 
**timings** | [**models::FinishedHttpTestResultAllOfTimings**](FinishedHttpTestResult_allOf_timings.md) |  | 
**tls** | Option<[**models::TlsCertificate**](TlsCertificate.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


