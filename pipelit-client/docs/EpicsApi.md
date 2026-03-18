# \EpicsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_delete_epics_api_v1_epics_batch_delete_post**](EpicsApi.md#batch_delete_epics_api_v1_epics_batch_delete_post) | **POST** /api/v1/epics/batch-delete/ | Batch Delete Epics
[**create_epic_api_v1_epics_post**](EpicsApi.md#create_epic_api_v1_epics_post) | **POST** /api/v1/epics/ | Create Epic
[**delete_epic_api_v1_epics_epic_id_delete**](EpicsApi.md#delete_epic_api_v1_epics_epic_id_delete) | **DELETE** /api/v1/epics/{epic_id}/ | Delete Epic
[**get_epic_api_v1_epics_epic_id_get**](EpicsApi.md#get_epic_api_v1_epics_epic_id_get) | **GET** /api/v1/epics/{epic_id}/ | Get Epic
[**list_epic_tasks_api_v1_epics_epic_id_tasks_get**](EpicsApi.md#list_epic_tasks_api_v1_epics_epic_id_tasks_get) | **GET** /api/v1/epics/{epic_id}/tasks/ | List Epic Tasks
[**list_epics_api_v1_epics_get**](EpicsApi.md#list_epics_api_v1_epics_get) | **GET** /api/v1/epics/ | List Epics
[**update_epic_api_v1_epics_epic_id_patch**](EpicsApi.md#update_epic_api_v1_epics_epic_id_patch) | **PATCH** /api/v1/epics/{epic_id}/ | Update Epic



## batch_delete_epics_api_v1_epics_batch_delete_post

> batch_delete_epics_api_v1_epics_batch_delete_post(batch_delete_epics_in)
Batch Delete Epics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_epics_in** | [**BatchDeleteEpicsIn**](BatchDeleteEpicsIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_epic_api_v1_epics_post

> models::EpicOut create_epic_api_v1_epics_post(epic_create)
Create Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_create** | [**EpicCreate**](EpicCreate.md) |  | [required] |

### Return type

[**models::EpicOut**](EpicOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_epic_api_v1_epics_epic_id_delete

> delete_epic_api_v1_epics_epic_id_delete(epic_id)
Delete Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_epic_api_v1_epics_epic_id_get

> models::EpicOut get_epic_api_v1_epics_epic_id_get(epic_id)
Get Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_id** | **String** |  | [required] |

### Return type

[**models::EpicOut**](EpicOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_epic_tasks_api_v1_epics_epic_id_tasks_get

> serde_json::Value list_epic_tasks_api_v1_epics_epic_id_tasks_get(epic_id, limit, offset)
List Epic Tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_id** | **String** |  | [required] |
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


## list_epics_api_v1_epics_get

> serde_json::Value list_epics_api_v1_epics_get(limit, offset, status, tags)
List Epics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 50]
**offset** | Option<**i32**> |  |  |[default to 0]
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


## update_epic_api_v1_epics_epic_id_patch

> models::EpicOut update_epic_api_v1_epics_epic_id_patch(epic_id, epic_update)
Update Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_id** | **String** |  | [required] |
**epic_update** | [**EpicUpdate**](EpicUpdate.md) |  | [required] |

### Return type

[**models::EpicOut**](EpicOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

