# \ExecutionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_delete_executions_api_v1_executions_batch_delete_post**](ExecutionsApi.md#batch_delete_executions_api_v1_executions_batch_delete_post) | **POST** /api/v1/executions/batch-delete/ | Batch Delete Executions
[**cancel_execution_api_v1_executions_execution_id_cancel_post**](ExecutionsApi.md#cancel_execution_api_v1_executions_execution_id_cancel_post) | **POST** /api/v1/executions/{execution_id}/cancel/ | Cancel Execution
[**get_execution_api_v1_executions_execution_id_get**](ExecutionsApi.md#get_execution_api_v1_executions_execution_id_get) | **GET** /api/v1/executions/{execution_id}/ | Get Execution
[**list_executions_api_v1_executions_get**](ExecutionsApi.md#list_executions_api_v1_executions_get) | **GET** /api/v1/executions/ | List Executions



## batch_delete_executions_api_v1_executions_batch_delete_post

> batch_delete_executions_api_v1_executions_batch_delete_post(batch_delete_executions_in)
Batch Delete Executions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_executions_in** | [**BatchDeleteExecutionsIn**](BatchDeleteExecutionsIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_execution_api_v1_executions_execution_id_cancel_post

> models::ExecutionOut cancel_execution_api_v1_executions_execution_id_cancel_post(execution_id)
Cancel Execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution_id** | **String** |  | [required] |

### Return type

[**models::ExecutionOut**](ExecutionOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_execution_api_v1_executions_execution_id_get

> models::ExecutionDetailOut get_execution_api_v1_executions_execution_id_get(execution_id)
Get Execution

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**execution_id** | **String** |  | [required] |

### Return type

[**models::ExecutionDetailOut**](ExecutionDetailOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_executions_api_v1_executions_get

> serde_json::Value list_executions_api_v1_executions_get(workflow_slug, status, limit, offset)
List Executions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_slug** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 50]
**offset** | Option<**i32**> |  |  |[default to 0]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

