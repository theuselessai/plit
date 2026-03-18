# NodeOut

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**node_id** | **String** |  | 
**label** | Option<**String**> |  | [optional]
**component_type** | **ComponentType** |  (enum: categorizer, router, extractor, ai_model, agent, deep_agent, switch, run_command, get_totp_code, platform_api, whoami, epic_tools, task_tools, spawn_and_await, workflow_create, workflow_discover, scheduler_tools, system_health, human_confirmation, workflow, code, code_execute, loop, wait, merge, filter, error_handler, output_parser, memory_read, memory_write, identify_user, trigger_telegram, trigger_schedule, trigger_manual, trigger_workflow, trigger_error, trigger_chat, skill) | 
**is_entry_point** | **bool** |  | 
**interrupt_before** | **bool** |  | 
**interrupt_after** | **bool** |  | 
**position_x** | **i32** |  | 
**position_y** | **i32** |  | 
**config** | [**models::ComponentConfigData**](ComponentConfigData.md) |  | 
**subworkflow_id** | Option<**i32**> |  | [optional]
**code_block_id** | Option<**i32**> |  | [optional]
**updated_at** | **String** |  | 
**schedule_job** | Option<[**models::ScheduleJobInfo**](ScheduleJobInfo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


