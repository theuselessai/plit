# \CredentialsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_credential_api_v1_credentials_credential_id_activate_post**](CredentialsApi.md#activate_credential_api_v1_credentials_credential_id_activate_post) | **POST** /api/v1/credentials/{credential_id}/activate/ | Activate Credential
[**batch_delete_credentials_api_v1_credentials_batch_delete_post**](CredentialsApi.md#batch_delete_credentials_api_v1_credentials_batch_delete_post) | **POST** /api/v1/credentials/batch-delete/ | Batch Delete Credentials
[**create_credential_api_v1_credentials_post**](CredentialsApi.md#create_credential_api_v1_credentials_post) | **POST** /api/v1/credentials/ | Create Credential
[**deactivate_credential_api_v1_credentials_credential_id_deactivate_post**](CredentialsApi.md#deactivate_credential_api_v1_credentials_credential_id_deactivate_post) | **POST** /api/v1/credentials/{credential_id}/deactivate/ | Deactivate Credential
[**delete_credential_api_v1_credentials_credential_id_delete**](CredentialsApi.md#delete_credential_api_v1_credentials_credential_id_delete) | **DELETE** /api/v1/credentials/{credential_id}/ | Delete Credential
[**get_credential_api_v1_credentials_credential_id_get**](CredentialsApi.md#get_credential_api_v1_credentials_credential_id_get) | **GET** /api/v1/credentials/{credential_id}/ | Get Credential
[**list_credential_models_api_v1_credentials_credential_id_models_get**](CredentialsApi.md#list_credential_models_api_v1_credentials_credential_id_models_get) | **GET** /api/v1/credentials/{credential_id}/models/ | List Credential Models
[**list_credentials_api_v1_credentials_get**](CredentialsApi.md#list_credentials_api_v1_credentials_get) | **GET** /api/v1/credentials/ | List Credentials
[**test_credential_api_v1_credentials_credential_id_test_post**](CredentialsApi.md#test_credential_api_v1_credentials_credential_id_test_post) | **POST** /api/v1/credentials/{credential_id}/test/ | Test Credential
[**update_credential_api_v1_credentials_credential_id_patch**](CredentialsApi.md#update_credential_api_v1_credentials_credential_id_patch) | **PATCH** /api/v1/credentials/{credential_id}/ | Update Credential



## activate_credential_api_v1_credentials_credential_id_activate_post

> serde_json::Value activate_credential_api_v1_credentials_credential_id_activate_post(credential_id)
Activate Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_delete_credentials_api_v1_credentials_batch_delete_post

> batch_delete_credentials_api_v1_credentials_batch_delete_post(batch_delete_credentials_in)
Batch Delete Credentials

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_delete_credentials_in** | [**BatchDeleteCredentialsIn**](BatchDeleteCredentialsIn.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_credential_api_v1_credentials_post

> models::CredentialOut create_credential_api_v1_credentials_post(credential_in)
Create Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_in** | [**CredentialIn**](CredentialIn.md) |  | [required] |

### Return type

[**models::CredentialOut**](CredentialOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## deactivate_credential_api_v1_credentials_credential_id_deactivate_post

> serde_json::Value deactivate_credential_api_v1_credentials_credential_id_deactivate_post(credential_id)
Deactivate Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_credential_api_v1_credentials_credential_id_delete

> delete_credential_api_v1_credentials_credential_id_delete(credential_id)
Delete Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_credential_api_v1_credentials_credential_id_get

> models::CredentialOut get_credential_api_v1_credentials_credential_id_get(credential_id)
Get Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i32** |  | [required] |

### Return type

[**models::CredentialOut**](CredentialOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_credential_models_api_v1_credentials_credential_id_models_get

> Vec<models::CredentialModelOut> list_credential_models_api_v1_credentials_credential_id_models_get(credential_id)
List Credential Models

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i32** |  | [required] |

### Return type

[**Vec<models::CredentialModelOut>**](CredentialModelOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_credentials_api_v1_credentials_get

> serde_json::Value list_credentials_api_v1_credentials_get(limit, offset)
List Credentials

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


## test_credential_api_v1_credentials_credential_id_test_post

> models::CredentialTestOut test_credential_api_v1_credentials_credential_id_test_post(credential_id)
Test Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i32** |  | [required] |

### Return type

[**models::CredentialTestOut**](CredentialTestOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_credential_api_v1_credentials_credential_id_patch

> models::CredentialOut update_credential_api_v1_credentials_credential_id_patch(credential_id, credential_update)
Update Credential

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**credential_id** | **i32** |  | [required] |
**credential_update** | [**CredentialUpdate**](CredentialUpdate.md) |  | [required] |

### Return type

[**models::CredentialOut**](CredentialOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

