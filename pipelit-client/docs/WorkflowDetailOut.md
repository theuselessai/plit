# WorkflowDetailOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**name** | **String** |  | 
**slug** | **String** |  | 
**description** | **String** |  | 
**is_active** | **bool** |  | 
**is_public** | **bool** |  | 
**is_default** | **bool** |  | 
**tags** | Option<**Vec<String>**> |  | [optional]
**error_handler_workflow_id** | Option<**i32**> |  | [optional]
**max_execution_seconds** | Option<**i32**> |  | [optional][default to 600]
**input_schema** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**output_schema** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**node_count** | Option<**i32**> |  | [optional][default to 0]
**edge_count** | Option<**i32**> |  | [optional][default to 0]
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**nodes** | Option<[**Vec<models::NodeOut>**](NodeOut.md)> |  | [optional][default to []]
**edges** | Option<[**Vec<models::EdgeOut>**](EdgeOut.md)> |  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


