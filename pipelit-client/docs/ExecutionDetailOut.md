# ExecutionDetailOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**execution_id** | **String** |  | 
**workflow_slug** | **String** |  | 
**status** | **String** |  | 
**error_message** | Option<**String**> |  | [optional][default to ]
**started_at** | Option<**String**> |  | [optional]
**completed_at** | Option<**String**> |  | [optional]
**total_tokens** | Option<**i32**> |  | [optional][default to 0]
**total_cost_usd** | Option<**f64**> |  | [optional][default to 0.0]
**llm_calls** | Option<**i32**> |  | [optional][default to 0]
**final_output** | Option<[**models::AnyOfLessThanGreaterThan**](AnyOf.md)> |  | [optional]
**trigger_payload** | Option<[**models::AnyOfLessThanGreaterThan**](AnyOf.md)> |  | [optional]
**logs** | Option<[**Vec<models::ExecutionLogOut>**](ExecutionLogOut.md)> |  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


