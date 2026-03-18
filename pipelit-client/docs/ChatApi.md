# \ChatApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_chat_history_api_v1_workflows_slug_chat_history_delete**](ChatApi.md#delete_chat_history_api_v1_workflows_slug_chat_history_delete) | **DELETE** /api/v1/workflows/{slug}/chat/history | Delete Chat History
[**delete_chat_history_api_v1_workflows_slug_chat_history_delete_0**](ChatApi.md#delete_chat_history_api_v1_workflows_slug_chat_history_delete_0) | **DELETE** /api/v1/workflows/{slug}/chat/history | Delete Chat History
[**get_chat_history_api_v1_workflows_slug_chat_history_get**](ChatApi.md#get_chat_history_api_v1_workflows_slug_chat_history_get) | **GET** /api/v1/workflows/{slug}/chat/history | Get Chat History
[**get_chat_history_api_v1_workflows_slug_chat_history_get_0**](ChatApi.md#get_chat_history_api_v1_workflows_slug_chat_history_get_0) | **GET** /api/v1/workflows/{slug}/chat/history | Get Chat History
[**send_chat_message_api_v1_workflows_slug_chat_post**](ChatApi.md#send_chat_message_api_v1_workflows_slug_chat_post) | **POST** /api/v1/workflows/{slug}/chat/ | Send Chat Message
[**send_chat_message_api_v1_workflows_slug_chat_post_0**](ChatApi.md#send_chat_message_api_v1_workflows_slug_chat_post_0) | **POST** /api/v1/workflows/{slug}/chat/ | Send Chat Message



## delete_chat_history_api_v1_workflows_slug_chat_history_delete

> delete_chat_history_api_v1_workflows_slug_chat_history_delete(slug)
Delete Chat History

Delete chat history from LangGraph checkpoints for this workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chat_history_api_v1_workflows_slug_chat_history_delete_0

> delete_chat_history_api_v1_workflows_slug_chat_history_delete_0(slug)
Delete Chat History

Delete chat history from LangGraph checkpoints for this workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_history_api_v1_workflows_slug_chat_history_get

> serde_json::Value get_chat_history_api_v1_workflows_slug_chat_history_get(slug, limit, before)
Get Chat History

Load chat history from LangGraph checkpoints.  Args:     slug: Workflow slug     limit: Max messages to return (default 10)     before: ISO datetime string - only return messages before this time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 10]
**before** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_history_api_v1_workflows_slug_chat_history_get_0

> serde_json::Value get_chat_history_api_v1_workflows_slug_chat_history_get_0(slug, limit, before)
Get Chat History

Load chat history from LangGraph checkpoints.  Args:     slug: Workflow slug     limit: Max messages to return (default 10)     before: ISO datetime string - only return messages before this time

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 10]
**before** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_chat_message_api_v1_workflows_slug_chat_post

> models::ChatMessageOut send_chat_message_api_v1_workflows_slug_chat_post(slug, chat_message_in)
Send Chat Message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**chat_message_in** | [**ChatMessageIn**](ChatMessageIn.md) |  | [required] |

### Return type

[**models::ChatMessageOut**](ChatMessageOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_chat_message_api_v1_workflows_slug_chat_post_0

> models::ChatMessageOut send_chat_message_api_v1_workflows_slug_chat_post_0(slug, chat_message_in)
Send Chat Message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**chat_message_in** | [**ChatMessageIn**](ChatMessageIn.md) |  | [required] |

### Return type

[**models::ChatMessageOut**](ChatMessageOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

