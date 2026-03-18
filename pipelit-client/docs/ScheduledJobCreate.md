# ScheduledJobCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional][default to ]
**workflow_id** | **i32** |  | 
**trigger_node_id** | Option<**String**> |  | [optional]
**interval_seconds** | **i32** |  | 
**total_repeats** | Option<**i32**> |  | [optional][default to 0]
**max_retries** | Option<**i32**> |  | [optional][default to 3]
**timeout_seconds** | Option<**i32**> |  | [optional][default to 600]
**trigger_payload** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


