# \InboundApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**inbound_webhook_api_v1_inbound_post**](InboundApi.md#inbound_webhook_api_v1_inbound_post) | **POST** /api/v1/inbound | Inbound Webhook
[**inbound_webhook_api_v1_inbound_post_0**](InboundApi.md#inbound_webhook_api_v1_inbound_post_0) | **POST** /api/v1/inbound | Inbound Webhook



## inbound_webhook_api_v1_inbound_post

> serde_json::Value inbound_webhook_api_v1_inbound_post(gateway_inbound_message)
Inbound Webhook

Receive a normalized message from msg-gateway and dispatch to workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_inbound_message** | [**GatewayInboundMessage**](GatewayInboundMessage.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inbound_webhook_api_v1_inbound_post_0

> serde_json::Value inbound_webhook_api_v1_inbound_post_0(gateway_inbound_message)
Inbound Webhook

Receive a normalized message from msg-gateway and dispatch to workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gateway_inbound_message** | [**GatewayInboundMessage**](GatewayInboundMessage.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

