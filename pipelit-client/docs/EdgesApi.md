# \EdgesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_edge_api_v1_workflows_slug_edges_post**](EdgesApi.md#create_edge_api_v1_workflows_slug_edges_post) | **POST** /api/v1/workflows/{slug}/edges/ | Create Edge
[**create_node_api_v1_workflows_slug_nodes_post**](EdgesApi.md#create_node_api_v1_workflows_slug_nodes_post) | **POST** /api/v1/workflows/{slug}/nodes/ | Create Node
[**delete_edge_api_v1_workflows_slug_edges_edge_id_delete**](EdgesApi.md#delete_edge_api_v1_workflows_slug_edges_edge_id_delete) | **DELETE** /api/v1/workflows/{slug}/edges/{edge_id}/ | Delete Edge
[**delete_node_api_v1_workflows_slug_nodes_node_id_delete**](EdgesApi.md#delete_node_api_v1_workflows_slug_nodes_node_id_delete) | **DELETE** /api/v1/workflows/{slug}/nodes/{node_id}/ | Delete Node
[**list_edges_api_v1_workflows_slug_edges_get**](EdgesApi.md#list_edges_api_v1_workflows_slug_edges_get) | **GET** /api/v1/workflows/{slug}/edges/ | List Edges
[**list_nodes_api_v1_workflows_slug_nodes_get**](EdgesApi.md#list_nodes_api_v1_workflows_slug_nodes_get) | **GET** /api/v1/workflows/{slug}/nodes/ | List Nodes
[**schedule_pause_api_v1_workflows_slug_nodes_node_id_schedule_pause_post**](EdgesApi.md#schedule_pause_api_v1_workflows_slug_nodes_node_id_schedule_pause_post) | **POST** /api/v1/workflows/{slug}/nodes/{node_id}/schedule/pause/ | Schedule Pause
[**schedule_start_api_v1_workflows_slug_nodes_node_id_schedule_start_post**](EdgesApi.md#schedule_start_api_v1_workflows_slug_nodes_node_id_schedule_start_post) | **POST** /api/v1/workflows/{slug}/nodes/{node_id}/schedule/start/ | Schedule Start
[**schedule_stop_api_v1_workflows_slug_nodes_node_id_schedule_stop_post**](EdgesApi.md#schedule_stop_api_v1_workflows_slug_nodes_node_id_schedule_stop_post) | **POST** /api/v1/workflows/{slug}/nodes/{node_id}/schedule/stop/ | Schedule Stop
[**update_edge_api_v1_workflows_slug_edges_edge_id_patch**](EdgesApi.md#update_edge_api_v1_workflows_slug_edges_edge_id_patch) | **PATCH** /api/v1/workflows/{slug}/edges/{edge_id}/ | Update Edge
[**update_node_api_v1_workflows_slug_nodes_node_id_patch**](EdgesApi.md#update_node_api_v1_workflows_slug_nodes_node_id_patch) | **PATCH** /api/v1/workflows/{slug}/nodes/{node_id}/ | Update Node



## create_edge_api_v1_workflows_slug_edges_post

> models::EdgeOut create_edge_api_v1_workflows_slug_edges_post(slug, edge_in)
Create Edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**edge_in** | [**EdgeIn**](EdgeIn.md) |  | [required] |

### Return type

[**models::EdgeOut**](EdgeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_node_api_v1_workflows_slug_nodes_post

> models::NodeOut create_node_api_v1_workflows_slug_nodes_post(slug, node_in)
Create Node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**node_in** | [**NodeIn**](NodeIn.md) |  | [required] |

### Return type

[**models::NodeOut**](NodeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_edge_api_v1_workflows_slug_edges_edge_id_delete

> delete_edge_api_v1_workflows_slug_edges_edge_id_delete(slug, edge_id)
Delete Edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**edge_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_node_api_v1_workflows_slug_nodes_node_id_delete

> delete_node_api_v1_workflows_slug_nodes_node_id_delete(slug, node_id)
Delete Node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_edges_api_v1_workflows_slug_edges_get

> Vec<models::EdgeOut> list_edges_api_v1_workflows_slug_edges_get(slug)
List Edges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**Vec<models::EdgeOut>**](EdgeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nodes_api_v1_workflows_slug_nodes_get

> Vec<models::NodeOut> list_nodes_api_v1_workflows_slug_nodes_get(slug)
List Nodes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |

### Return type

[**Vec<models::NodeOut>**](NodeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_pause_api_v1_workflows_slug_nodes_node_id_schedule_pause_post

> models::NodeOut schedule_pause_api_v1_workflows_slug_nodes_node_id_schedule_pause_post(slug, node_id)
Schedule Pause

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

[**models::NodeOut**](NodeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_start_api_v1_workflows_slug_nodes_node_id_schedule_start_post

> models::NodeOut schedule_start_api_v1_workflows_slug_nodes_node_id_schedule_start_post(slug, node_id)
Schedule Start

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

[**models::NodeOut**](NodeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_stop_api_v1_workflows_slug_nodes_node_id_schedule_stop_post

> models::NodeOut schedule_stop_api_v1_workflows_slug_nodes_node_id_schedule_stop_post(slug, node_id)
Schedule Stop

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**node_id** | **String** |  | [required] |

### Return type

[**models::NodeOut**](NodeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_edge_api_v1_workflows_slug_edges_edge_id_patch

> models::EdgeOut update_edge_api_v1_workflows_slug_edges_edge_id_patch(slug, edge_id, edge_update)
Update Edge

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**edge_id** | **i32** |  | [required] |
**edge_update** | [**EdgeUpdate**](EdgeUpdate.md) |  | [required] |

### Return type

[**models::EdgeOut**](EdgeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_node_api_v1_workflows_slug_nodes_node_id_patch

> models::NodeOut update_node_api_v1_workflows_slug_nodes_node_id_patch(slug, node_id, node_update)
Update Node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**slug** | **String** |  | [required] |
**node_id** | **String** |  | [required] |
**node_update** | [**NodeUpdate**](NodeUpdate.md) |  | [required] |

### Return type

[**models::NodeOut**](NodeOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

