# \MemoriesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_delete_checkpoints_api_v1_memories_checkpoints_batch_delete_post**](MemoriesApi.md#batch_delete_checkpoints_api_v1_memories_checkpoints_batch_delete_post) | **POST** /api/v1/memories/checkpoints/batch-delete/ | Batch Delete Checkpoints
[**batch_delete_episodes_api_v1_memories_episodes_batch_delete_post**](MemoriesApi.md#batch_delete_episodes_api_v1_memories_episodes_batch_delete_post) | **POST** /api/v1/memories/episodes/batch-delete/ | Batch Delete Episodes
[**batch_delete_facts_api_v1_memories_facts_batch_delete_post**](MemoriesApi.md#batch_delete_facts_api_v1_memories_facts_batch_delete_post) | **POST** /api/v1/memories/facts/batch-delete/ | Batch Delete Facts
[**batch_delete_procedures_api_v1_memories_procedures_batch_delete_post**](MemoriesApi.md#batch_delete_procedures_api_v1_memories_procedures_batch_delete_post) | **POST** /api/v1/memories/procedures/batch-delete/ | Batch Delete Procedures
[**batch_delete_users_api_v1_memories_users_batch_delete_post**](MemoriesApi.md#batch_delete_users_api_v1_memories_users_batch_delete_post) | **POST** /api/v1/memories/users/batch-delete/ | Batch Delete Users
[**list_checkpoints_api_v1_memories_checkpoints_get**](MemoriesApi.md#list_checkpoints_api_v1_memories_checkpoints_get) | **GET** /api/v1/memories/checkpoints/ | List Checkpoints
[**list_episodes_api_v1_memories_episodes_get**](MemoriesApi.md#list_episodes_api_v1_memories_episodes_get) | **GET** /api/v1/memories/episodes/ | List Episodes
[**list_facts_api_v1_memories_facts_get**](MemoriesApi.md#list_facts_api_v1_memories_facts_get) | **GET** /api/v1/memories/facts/ | List Facts
[**list_procedures_api_v1_memories_procedures_get**](MemoriesApi.md#list_procedures_api_v1_memories_procedures_get) | **GET** /api/v1/memories/procedures/ | List Procedures
[**list_users_api_v1_memories_users_get**](MemoriesApi.md#list_users_api_v1_memories_users_get) | **GET** /api/v1/memories/users/ | List Users



## batch_delete_checkpoints_api_v1_memories_checkpoints_batch_delete_post

> batch_delete_checkpoints_api_v1_memories_checkpoints_batch_delete_post(batch_delete_checkpoints_in)
Batch Delete Checkpoints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_checkpoints_in** | [**BatchDeleteCheckpointsIn**](BatchDeleteCheckpointsIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_delete_episodes_api_v1_memories_episodes_batch_delete_post

> batch_delete_episodes_api_v1_memories_episodes_batch_delete_post(batch_delete_episodes_in)
Batch Delete Episodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_episodes_in** | [**BatchDeleteEpisodesIn**](BatchDeleteEpisodesIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_delete_facts_api_v1_memories_facts_batch_delete_post

> batch_delete_facts_api_v1_memories_facts_batch_delete_post(batch_delete_facts_in)
Batch Delete Facts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_facts_in** | [**BatchDeleteFactsIn**](BatchDeleteFactsIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_delete_procedures_api_v1_memories_procedures_batch_delete_post

> batch_delete_procedures_api_v1_memories_procedures_batch_delete_post(batch_delete_procedures_in)
Batch Delete Procedures

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_procedures_in** | [**BatchDeleteProceduresIn**](BatchDeleteProceduresIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_delete_users_api_v1_memories_users_batch_delete_post

> batch_delete_users_api_v1_memories_users_batch_delete_post(batch_delete_users_in)
Batch Delete Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_users_in** | [**BatchDeleteUsersIn**](BatchDeleteUsersIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_checkpoints_api_v1_memories_checkpoints_get

> serde_json::Value list_checkpoints_api_v1_memories_checkpoints_get(thread_id, limit, offset)
List Checkpoints

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | Option<**String**> |  |  |
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


## list_episodes_api_v1_memories_episodes_get

> serde_json::Value list_episodes_api_v1_memories_episodes_get(agent_id, limit, offset)
List Episodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | Option<**String**> |  |  |
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


## list_facts_api_v1_memories_facts_get

> serde_json::Value list_facts_api_v1_memories_facts_get(scope, fact_type, limit, offset)
List Facts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | Option<**String**> |  |  |
**fact_type** | Option<**String**> |  |  |
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


## list_procedures_api_v1_memories_procedures_get

> serde_json::Value list_procedures_api_v1_memories_procedures_get(agent_id, limit, offset)
List Procedures

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**agent_id** | Option<**String**> |  |  |
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


## list_users_api_v1_memories_users_get

> serde_json::Value list_users_api_v1_memories_users_get(limit, offset)
List Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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

