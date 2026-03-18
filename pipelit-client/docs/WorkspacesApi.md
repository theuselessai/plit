# \WorkspacesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**batch_delete_workspaces_api_v1_workspaces_batch_delete_post**](WorkspacesApi.md#batch_delete_workspaces_api_v1_workspaces_batch_delete_post) | **POST** /api/v1/workspaces/batch-delete/ | Batch Delete Workspaces
[**create_workspace_api_v1_workspaces_post**](WorkspacesApi.md#create_workspace_api_v1_workspaces_post) | **POST** /api/v1/workspaces/ | Create Workspace
[**delete_workspace_api_v1_workspaces_workspace_id_delete**](WorkspacesApi.md#delete_workspace_api_v1_workspaces_workspace_id_delete) | **DELETE** /api/v1/workspaces/{workspace_id}/ | Delete Workspace
[**get_workspace_api_v1_workspaces_workspace_id_get**](WorkspacesApi.md#get_workspace_api_v1_workspaces_workspace_id_get) | **GET** /api/v1/workspaces/{workspace_id}/ | Get Workspace
[**list_workspaces_api_v1_workspaces_get**](WorkspacesApi.md#list_workspaces_api_v1_workspaces_get) | **GET** /api/v1/workspaces/ | List Workspaces
[**reset_rootfs_api_v1_workspaces_workspace_id_reset_rootfs_post**](WorkspacesApi.md#reset_rootfs_api_v1_workspaces_workspace_id_reset_rootfs_post) | **POST** /api/v1/workspaces/{workspace_id}/reset-rootfs/ | Reset Rootfs
[**reset_workspace_api_v1_workspaces_workspace_id_reset_post**](WorkspacesApi.md#reset_workspace_api_v1_workspaces_workspace_id_reset_post) | **POST** /api/v1/workspaces/{workspace_id}/reset/ | Reset Workspace
[**update_workspace_api_v1_workspaces_workspace_id_patch**](WorkspacesApi.md#update_workspace_api_v1_workspaces_workspace_id_patch) | **PATCH** /api/v1/workspaces/{workspace_id}/ | Update Workspace



## batch_delete_workspaces_api_v1_workspaces_batch_delete_post

> batch_delete_workspaces_api_v1_workspaces_batch_delete_post(batch_delete_workspaces_in)
Batch Delete Workspaces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_workspaces_in** | [**BatchDeleteWorkspacesIn**](BatchDeleteWorkspacesIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workspace_api_v1_workspaces_post

> models::WorkspaceOut create_workspace_api_v1_workspaces_post(workspace_in)
Create Workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_in** | [**WorkspaceIn**](WorkspaceIn.md) |  | [required] |

### Return type

[**models::WorkspaceOut**](WorkspaceOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workspace_api_v1_workspaces_workspace_id_delete

> delete_workspace_api_v1_workspaces_workspace_id_delete(workspace_id)
Delete Workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspace_api_v1_workspaces_workspace_id_get

> models::WorkspaceOut get_workspace_api_v1_workspaces_workspace_id_get(workspace_id)
Get Workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** |  | [required] |

### Return type

[**models::WorkspaceOut**](WorkspaceOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_workspaces_api_v1_workspaces_get

> serde_json::Value list_workspaces_api_v1_workspaces_get(limit, offset)
List Workspaces

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


## reset_rootfs_api_v1_workspaces_workspace_id_reset_rootfs_post

> serde_json::Value reset_rootfs_api_v1_workspaces_workspace_id_reset_rootfs_post(workspace_id)
Reset Rootfs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_workspace_api_v1_workspaces_workspace_id_reset_post

> serde_json::Value reset_workspace_api_v1_workspaces_workspace_id_reset_post(workspace_id)
Reset Workspace

Reset a workspace by deleting everything inside its directory and re-creating it empty.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workspace_api_v1_workspaces_workspace_id_patch

> models::WorkspaceOut update_workspace_api_v1_workspaces_workspace_id_patch(workspace_id, workspace_update)
Update Workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **i32** |  | [required] |
**workspace_update** | [**WorkspaceUpdate**](WorkspaceUpdate.md) |  | [required] |

### Return type

[**models::WorkspaceOut**](WorkspaceOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

