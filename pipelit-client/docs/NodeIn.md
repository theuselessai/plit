# NodeIn

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**node_id** | Option<**String**> |  | [optional]
**label** | Option<**String**> |  | [optional]
**component_type** | **ComponentType** |  (enum: categorizer, router, extractor, ai_model, agent, deep_agent, switch, run_command, get_totp_code, platform_api, whoami, epic_tools, task_tools, spawn_and_await, workflow_create, workflow_discover, scheduler_tools, system_health, human_confirmation, workflow, code, code_execute, loop, wait, merge, filter, error_handler, output_parser, memory_read, memory_write, identify_user, trigger_telegram, trigger_schedule, trigger_manual, trigger_workflow, trigger_error, trigger_chat, skill) | 
**is_entry_point** | Option<**bool**> |  | [optional][default to false]
**interrupt_before** | Option<**bool**> |  | [optional][default to false]
**interrupt_after** | Option<**bool**> |  | [optional][default to false]
**position_x** | Option<**i32**> |  | [optional][default to 0]
**position_y** | Option<**i32**> |  | [optional][default to 0]
**config** | Option<[**models::ComponentConfigData**](ComponentConfigData.md)> |  | [optional][default to {system_prompt=, extra_config={}, model_name=, is_active=true, priority=0, trigger_config={}}]
**subworkflow_id** | Option<**i32**> |  | [optional]
**code_block_id** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


