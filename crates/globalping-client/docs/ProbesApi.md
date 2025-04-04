# \ProbesApi

All URIs are relative to *https://api.globalping.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_probes**](ProbesApi.md#list_probes) | **GET** /v1/probes | List probes currently online



## list_probes

> Vec<models::Probe> list_probes()
List probes currently online

Returns a list of all probes currently online and their metadata, such as location and assigned tags.  > **Note**: Probes don't expose unique IDs that would allow you to explicitly select them. Instead, specify the requested location or an ID of an existing measurement when creating new measurements. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Probe>**](Probe.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

