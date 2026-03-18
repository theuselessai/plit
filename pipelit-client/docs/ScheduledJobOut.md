# ScheduledJobOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional][default to ]
**workflow_id** | **i32** |  | 
**trigger_node_id** | Option<**String**> |  | [optional]
**user_profile_id** | **i32** |  | 
**interval_seconds** | **i32** |  | 
**total_repeats** | Option<**i32**> |  | [optional][default to 0]
**max_retries** | Option<**i32**> |  | [optional][default to 3]
**timeout_seconds** | Option<**i32**> |  | [optional][default to 600]
**trigger_payload** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**status** | Option<**String**> |  | [optional][default to active]
**current_repeat** | Option<**i32**> |  | [optional][default to 0]
**current_retry** | Option<**i32**> |  | [optional][default to 0]
**last_run_at** | Option<**String**> |  | [optional]
**next_run_at** | Option<**String**> |  | [optional]
**run_count** | Option<**i32**> |  | [optional][default to 0]
**error_count** | Option<**i32**> |  | [optional][default to 0]
**last_error** | Option<**String**> |  | [optional][default to ]
**created_at** | Option<**String**> |  | [optional]
**updated_at** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


