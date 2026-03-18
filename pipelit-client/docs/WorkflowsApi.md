# \WorkflowsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_delete_workflows_api_v1_workflows_batch_delete_post**](WorkflowsApi.md#batch_delete_workflows_api_v1_workflows_batch_delete_post) | **POST** /api/v1/workflows/batch-delete/ | Batch Delete Workflows
[**create_workflow_api_v1_workflows_post**](WorkflowsApi.md#create_workflow_api_v1_workflows_post) | **POST** /api/v1/workflows/ | Create Workflow
[**delete_workflow_api_v1_workflows_slug_delete**](WorkflowsApi.md#delete_workflow_api_v1_workflows_slug_delete) | **DELETE** /api/v1/workflows/{slug}/ | Delete Workflow
[**get_workflow_detail_api_v1_workflows_slug_get**](WorkflowsApi.md#get_workflow_detail_api_v1_workflows_slug_get) | **GET** /api/v1/workflows/{slug}/ | Get Workflow Detail
[**list_node_types_api_v1_workflows_node_types_get**](WorkflowsApi.md#list_node_types_api_v1_workflows_node_types_get) | **GET** /api/v1/workflows/node-types/ | List Node Types
[**list_workflows_api_v1_workflows_get**](WorkflowsApi.md#list_workflows_api_v1_workflows_get) | **GET** /api/v1/workflows/ | List Workflows
[**update_workflow_api_v1_workflows_slug_patch**](WorkflowsApi.md#update_workflow_api_v1_workflows_slug_patch) | **PATCH** /api/v1/workflows/{slug}/ | Update Workflow
[**validate_dsl_endpoint_api_v1_workflows_validate_dsl_post**](WorkflowsApi.md#validate_dsl_endpoint_api_v1_workflows_validate_dsl_post) | **POST** /api/v1/workflows/validate-dsl/ | Validate Dsl Endpoint
[**validate_workflow_api_v1_workflows_slug_validate_post**](WorkflowsApi.md#validate_workflow_api_v1_workflows_slug_validate_post) | **POST** /api/v1/workflows/{slug}/validate/ | Validate Workflow



## batch_delete_workflows_api_v1_workflows_batch_delete_post

> batch_delete_workflows_api_v1_workflows_batch_delete_post(batch_delete_workflows_in)
Batch Delete Workflows

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_workflows_in** | [**BatchDeleteWorkflowsIn**](BatchDeleteWorkflowsIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workflow_api_v1_workflows_post

> models::WorkflowOut create_workflow_api_v1_workflows_post(workflow_in)
Create Workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_in** | [**WorkflowIn**](WorkflowIn.md) |  | [required] |

### Return type

[**models::WorkflowOut**](WorkflowOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow_api_v1_workflows_slug_delete

> delete_workflow_api_v1_workflows_slug_delete(slug)
Delete Workflow

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


## get_workflow_detail_api_v1_workflows_slug_get

> models::WorkflowDetailOut get_workflow_detail_api_v1_workflows_slug_get(slug)
Get Workflow Detail

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**models::WorkflowDetailOut**](WorkflowDetailOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_node_types_api_v1_workflows_node_types_get

> serde_json::Value list_node_types_api_v1_workflows_node_types_get()
List Node Types

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workflows_api_v1_workflows_get

> serde_json::Value list_workflows_api_v1_workflows_get(limit, offset)
List Workflows

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


## update_workflow_api_v1_workflows_slug_patch

> models::WorkflowOut update_workflow_api_v1_workflows_slug_patch(slug, workflow_update)
Update Workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**workflow_update** | [**WorkflowUpdate**](WorkflowUpdate.md) |  | [required] |

### Return type

[**models::WorkflowOut**](WorkflowOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_dsl_endpoint_api_v1_workflows_validate_dsl_post

> serde_json::Value validate_dsl_endpoint_api_v1_workflows_validate_dsl_post(validate_dsl_in)
Validate Dsl Endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**validate_dsl_in** | [**ValidateDslIn**](ValidateDslIn.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_workflow_api_v1_workflows_slug_validate_post

> serde_json::Value validate_workflow_api_v1_workflows_slug_validate_post(slug)
Validate Workflow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

