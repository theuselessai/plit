# TaskCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**epic_id** | **String** |  | 
**title** | **String** |  | 
**description** | Option<**String**> |  | [optional][default to ]
**tags** | Option<**Vec<String>**> |  | [optional][default to []]
**depends_on** | Option<**Vec<String>**> |  | [optional][default to []]
**priority** | Option<**i32**> |  | [optional]
**workflow_slug** | Option<**String**> |  | [optional]
**estimated_tokens** | Option<**i32**> |  | [optional]
**max_retries** | Option<**i32**> |  | [optional][default to 2]
**requirements** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


