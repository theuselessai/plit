# \SchedulesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_delete_schedules_api_v1_schedules_batch_delete_post**](SchedulesApi.md#batch_delete_schedules_api_v1_schedules_batch_delete_post) | **POST** /api/v1/schedules/batch-delete/ | Batch Delete Schedules
[**create_schedule_api_v1_schedules_post**](SchedulesApi.md#create_schedule_api_v1_schedules_post) | **POST** /api/v1/schedules/ | Create Schedule
[**delete_schedule_api_v1_schedules_job_id_delete**](SchedulesApi.md#delete_schedule_api_v1_schedules_job_id_delete) | **DELETE** /api/v1/schedules/{job_id}/ | Delete Schedule
[**get_schedule_api_v1_schedules_job_id_get**](SchedulesApi.md#get_schedule_api_v1_schedules_job_id_get) | **GET** /api/v1/schedules/{job_id}/ | Get Schedule
[**list_schedules_api_v1_schedules_get**](SchedulesApi.md#list_schedules_api_v1_schedules_get) | **GET** /api/v1/schedules/ | List Schedules
[**pause_schedule_api_v1_schedules_job_id_pause_post**](SchedulesApi.md#pause_schedule_api_v1_schedules_job_id_pause_post) | **POST** /api/v1/schedules/{job_id}/pause/ | Pause Schedule
[**resume_schedule_api_v1_schedules_job_id_resume_post**](SchedulesApi.md#resume_schedule_api_v1_schedules_job_id_resume_post) | **POST** /api/v1/schedules/{job_id}/resume/ | Resume Schedule
[**update_schedule_api_v1_schedules_job_id_patch**](SchedulesApi.md#update_schedule_api_v1_schedules_job_id_patch) | **PATCH** /api/v1/schedules/{job_id}/ | Update Schedule



## batch_delete_schedules_api_v1_schedules_batch_delete_post

> batch_delete_schedules_api_v1_schedules_batch_delete_post(batch_delete_schedules_in)
Batch Delete Schedules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_schedules_in** | [**BatchDeleteSchedulesIn**](BatchDeleteSchedulesIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_schedule_api_v1_schedules_post

> models::ScheduledJobOut create_schedule_api_v1_schedules_post(scheduled_job_create)
Create Schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scheduled_job_create** | [**ScheduledJobCreate**](ScheduledJobCreate.md) |  | [required] |

### Return type

[**models::ScheduledJobOut**](ScheduledJobOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_schedule_api_v1_schedules_job_id_delete

> delete_schedule_api_v1_schedules_job_id_delete(job_id)
Delete Schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_schedule_api_v1_schedules_job_id_get

> models::ScheduledJobOut get_schedule_api_v1_schedules_job_id_get(job_id)
Get Schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::ScheduledJobOut**](ScheduledJobOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_schedules_api_v1_schedules_get

> serde_json::Value list_schedules_api_v1_schedules_get(limit, offset, status, workflow_id)
List Schedules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |[default to 50]
**offset** | Option<**i32**> |  |  |[default to 0]
**status** | Option<**String**> |  |  |
**workflow_id** | Option<**i32**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_schedule_api_v1_schedules_job_id_pause_post

> models::ScheduledJobOut pause_schedule_api_v1_schedules_job_id_pause_post(job_id)
Pause Schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::ScheduledJobOut**](ScheduledJobOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_schedule_api_v1_schedules_job_id_resume_post

> models::ScheduledJobOut resume_schedule_api_v1_schedules_job_id_resume_post(job_id)
Resume Schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |

### Return type

[**models::ScheduledJobOut**](ScheduledJobOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_schedule_api_v1_schedules_job_id_patch

> models::ScheduledJobOut update_schedule_api_v1_schedules_job_id_patch(job_id, scheduled_job_update)
Update Schedule

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** |  | [required] |
**scheduled_job_update** | [**ScheduledJobUpdate**](ScheduledJobUpdate.md) |  | [required] |

### Return type

[**models::ScheduledJobOut**](ScheduledJobOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

