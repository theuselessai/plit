# WorkflowIn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**slug** | **String** |  | 
**description** | Option<**String**> |  | [optional][default to ]
**is_active** | Option<**bool**> |  | [optional][default to true]
**is_public** | Option<**bool**> |  | [optional][default to false]
**is_default** | Option<**bool**> |  | [optional][default to false]
**tags** | Option<**Vec<String>**> |  | [optional]
**error_handler_workflow_id** | Option<**i32**> |  | [optional]
**max_execution_seconds** | Option<**i32**> |  | [optional][default to 600]
**input_schema** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**output_schema** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


