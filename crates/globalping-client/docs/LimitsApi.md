# \LimitsApi

All URIs are relative to *https://api.globalping.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_limits**](LimitsApi.md#get_limits) | **GET** /v1/limits | Get current rate limits



## get_limits

> models::Limits get_limits()
Get current rate limits

Returns rate limits for the current user (if authenticated) or IP address (if not authenticated). 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Limits**](Limits.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

