# ExecutionLogOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**node_id** | **String** |  | 
**status** | **String** |  | 
**input** | Option<[**models::AnyOfLessThanGreaterThan**](AnyOf.md)> |  | [optional]
**output** | Option<[**models::AnyOfLessThanGreaterThan**](AnyOf.md)> |  | [optional]
**error** | Option<**String**> |  | [optional][default to ]
**error_code** | Option<**String**> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**duration_ms** | Option<**i32**> |  | [optional][default to 0]
**timestamp** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


