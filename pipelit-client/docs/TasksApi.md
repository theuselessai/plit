# \TasksApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_delete_tasks_api_v1_tasks_batch_delete_post**](TasksApi.md#batch_delete_tasks_api_v1_tasks_batch_delete_post) | **POST** /api/v1/tasks/batch-delete/ | Batch Delete Tasks
[**create_task_api_v1_tasks_post**](TasksApi.md#create_task_api_v1_tasks_post) | **POST** /api/v1/tasks/ | Create Task
[**delete_task_api_v1_tasks_task_id_delete**](TasksApi.md#delete_task_api_v1_tasks_task_id_delete) | **DELETE** /api/v1/tasks/{task_id}/ | Delete Task
[**get_task_api_v1_tasks_task_id_get**](TasksApi.md#get_task_api_v1_tasks_task_id_get) | **GET** /api/v1/tasks/{task_id}/ | Get Task
[**list_tasks_api_v1_tasks_get**](TasksApi.md#list_tasks_api_v1_tasks_get) | **GET** /api/v1/tasks/ | List Tasks
[**update_task_api_v1_tasks_task_id_patch**](TasksApi.md#update_task_api_v1_tasks_task_id_patch) | **PATCH** /api/v1/tasks/{task_id}/ | Update Task



## batch_delete_tasks_api_v1_tasks_batch_delete_post

> batch_delete_tasks_api_v1_tasks_batch_delete_post(batch_delete_tasks_in)
Batch Delete Tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_tasks_in** | [**BatchDeleteTasksIn**](BatchDeleteTasksIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_task_api_v1_tasks_post

> models::TaskOut create_task_api_v1_tasks_post(task_create)
Create Task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_create** | [**TaskCreate**](TaskCreate.md) |  | [required] |

### Return type

[**models::TaskOut**](TaskOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_task_api_v1_tasks_task_id_delete

> delete_task_api_v1_tasks_task_id_delete(task_id)
Delete Task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task_api_v1_tasks_task_id_get

> models::TaskOut get_task_api_v1_tasks_task_id_get(task_id)
Get Task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |

### Return type

[**models::TaskOut**](TaskOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks_api_v1_tasks_get

> serde_json::Value list_tasks_api_v1_tasks_get(limit, offset, epic_id, status, tags)
List Tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 50]
**offset** | Option<**i32**> |  |  |[default to 0]
**epic_id** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**tags** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_api_v1_tasks_task_id_patch

> models::TaskOut update_task_api_v1_tasks_task_id_patch(task_id, task_update)
Update Task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |
**task_update** | [**TaskUpdate**](TaskUpdate.md) |  | [required] |

### Return type

[**models::TaskOut**](TaskOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

