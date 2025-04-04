# MeasurementHttpOptionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | Option<**String**> | An optional override for the `Host` header. The default value is based on the `target`.  | [optional]
**path** | Option<**String**> | The path portion of the URL. | [optional]
**query** | Option<**String**> | The query string portion of the URL. | [optional]
**method** | Option<**String**> | The HTTP method to use. | [optional][default to Head]
**headers** | Option<**std::collections::HashMap<String, String>**> | Additional request headers. Note that the `Host` and `User-Agent` are reserved and internally overridden.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


