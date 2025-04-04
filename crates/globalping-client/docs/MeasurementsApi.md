# \MeasurementsApi

All URIs are relative to *https://api.globalping.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_measurement**](MeasurementsApi.md#create_measurement) | **POST** /v1/measurements | Create a measurement
[**get_measurement**](MeasurementsApi.md#get_measurement) | **GET** /v1/measurements/{id} | Get a measurement by ID



## create_measurement

> models::CreateMeasurementResponse create_measurement(measurement_request)
Create a measurement

Creates a new measurement with parameters set in the request body. The measurement runs asynchronously and you can retrieve its current state at the URL returned in the `Location` header.  ### Client guidelines  - If the application is running in interactive mode, set the `inProgressUpdates` option to `true` to have the API   return partial results as soon as they are available. This allows the user to see the measurement progress in real time.   - If the application is interactive by default but also implements a \"CI\" mode for scripting, do not set the flag in the CI mode. - To perform multiple measurements using exactly the same probes, create a single measurement first, then pass its `id` in the `locations` field for the other measurements. - When you receive a `429` response, inform the user about their current rate limit status based on the response headers. Depending on the exact situation and on what your application supports, you may also suggest:   - Signing in or using an access token.   - Learning more about how to get additional credits at https://globalping.io/credits.   - Repeating the measurement with fewer probes. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**measurement_request** | Option<[**MeasurementRequest**](MeasurementRequest.md)> | Use the request body to set the measurement parameters. |  |

### Return type

[**models::CreateMeasurementResponse**](CreateMeasurementResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_measurement

> models::InlineObject2 get_measurement(id)
Get a measurement by ID

Returns the status and results of an existing measurement. Measurements are typically available for up to 7 days after creation.  > **Tip**: A link to this endpoint is returned in the `Location` response header when creating the measurement.  ### Client guidelines  As it can take a few seconds for a measurement to complete, you should use the following process for retrieving the results:   1. Request the measurement to retrieve its status.   2. If the status is `in-progress`, wait 500 milliseconds and start again at step 1. Note that it's important to wait 500 ms *after* receiving the response rather than using an \"every 500ms\" interval as for large measurements, the request itself may take a few hundred milliseconds to complete.   3. If the status is anything **other** than `in-progress`, stop. The measurement is no longer running, and its results are final.  > **Important**: Do not query the results of a single measurement more often than every 500 milliseconds. Sending more than two requests per second may trigger a rate limit and prevent you from accessing the results for a few seconds. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the measurement you want to retrieve. | [required] |

### Return type

[**models::InlineObject2**](inline_object_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

