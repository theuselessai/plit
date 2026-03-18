# GatewayInboundMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**route** | **std::collections::HashMap<String, serde_json::Value>** |  | 
**credential_id** | **String** |  | 
**source** | [**models::InboundSource**](InboundSource.md) |  | 
**text** | **String** |  | 
**attachments** | Option<[**Vec<models::InboundAttachment>**](InboundAttachment.md)> |  | [optional][default to []]
**timestamp** | **String** |  | 
**extra_data** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


