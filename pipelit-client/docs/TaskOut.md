# TaskOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**epic_id** | **String** |  | 
**title** | **String** |  | 
**description** | Option<**String**> |  | [optional][default to ]
**tags** | Option<**Vec<String>**> |  | [optional][default to []]
**created_by_node_id** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional][default to pending]
**priority** | Option<**i32**> |  | [optional][default to 2]
**workflow_id** | Option<**i32**> |  | [optional]
**workflow_slug** | Option<**String**> |  | [optional]
**execution_id** | Option<**String**> |  | [optional]
**workflow_source** | Option<**String**> |  | [optional][default to inline]
**depends_on** | Option<**Vec<String>**> |  | [optional][default to []]
**requirements** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**estimated_tokens** | Option<**i32**> |  | [optional]
**actual_tokens** | Option<**i32**> |  | [optional][default to 0]
**actual_usd** | Option<**f64**> |  | [optional][default to 0.0]
**llm_calls** | Option<**i32**> |  | [optional][default to 0]
**tool_invocations** | Option<**i32**> |  | [optional][default to 0]
**duration_ms** | Option<**i32**> |  | [optional][default to 0]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]
**started_at** | Option<**String**> |  | [optional]
**completed_at** | Option<**String**> |  | [optional]
**result_summary** | Option<**String**> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**retry_count** | Option<**i32**> |  | [optional][default to 0]
**max_retries** | Option<**i32**> |  | [optional][default to 2]
**notes** | Option<**Vec<serde_json::Value>**> |  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


