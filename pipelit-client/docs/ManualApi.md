# \ManualApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**execution_status_view_api_v1_executions_execution_id_status_get**](ManualApi.md#execution_status_view_api_v1_executions_execution_id_status_get) | **GET** /api/v1/executions/{execution_id}/status/ | Execution Status View
[**manual_execute_view_api_v1_workflows_workflow_slug_execute_post**](ManualApi.md#manual_execute_view_api_v1_workflows_workflow_slug_execute_post) | **POST** /api/v1/workflows/{workflow_slug}/execute/ | Manual Execute View



## execution_status_view_api_v1_executions_execution_id_status_get

> serde_json::Value execution_status_view_api_v1_executions_execution_id_status_get(execution_id)
Execution Status View

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manual_execute_view_api_v1_workflows_workflow_slug_execute_post

> serde_json::Value manual_execute_view_api_v1_workflows_workflow_slug_execute_post(workflow_slug, manual_execute_in)
Manual Execute View

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_slug** | **String** |  | [required] |
**manual_execute_in** | Option<[**ManualExecuteIn**](ManualExecuteIn.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

