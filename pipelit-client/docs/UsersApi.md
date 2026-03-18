# \UsersApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_own_key_api_v1_users_me_keys_post**](UsersApi.md#create_own_key_api_v1_users_me_keys_post) | **POST** /api/v1/users/me/keys | Create Own Key
[**create_user_api_v1_users_post**](UsersApi.md#create_user_api_v1_users_post) | **POST** /api/v1/users/ | Create User
[**create_user_key_api_v1_users_user_id_keys_post**](UsersApi.md#create_user_key_api_v1_users_user_id_keys_post) | **POST** /api/v1/users/{user_id}/keys | Create User Key
[**delete_user_api_v1_users_user_id_delete**](UsersApi.md#delete_user_api_v1_users_user_id_delete) | **DELETE** /api/v1/users/{user_id} | Delete User
[**get_own_profile_api_v1_users_me_get**](UsersApi.md#get_own_profile_api_v1_users_me_get) | **GET** /api/v1/users/me | Get Own Profile
[**get_user_api_v1_users_user_id_get**](UsersApi.md#get_user_api_v1_users_user_id_get) | **GET** /api/v1/users/{user_id} | Get User
[**list_own_keys_api_v1_users_me_keys_get**](UsersApi.md#list_own_keys_api_v1_users_me_keys_get) | **GET** /api/v1/users/me/keys | List Own Keys
[**list_user_keys_api_v1_users_user_id_keys_get**](UsersApi.md#list_user_keys_api_v1_users_user_id_keys_get) | **GET** /api/v1/users/{user_id}/keys | List User Keys
[**list_users_api_v1_users_get**](UsersApi.md#list_users_api_v1_users_get) | **GET** /api/v1/users/ | List Users
[**revoke_own_key_api_v1_users_me_keys_key_id_delete**](UsersApi.md#revoke_own_key_api_v1_users_me_keys_key_id_delete) | **DELETE** /api/v1/users/me/keys/{key_id} | Revoke Own Key
[**revoke_user_key_api_v1_users_user_id_keys_key_id_delete**](UsersApi.md#revoke_user_key_api_v1_users_user_id_keys_key_id_delete) | **DELETE** /api/v1/users/{user_id}/keys/{key_id} | Revoke User Key
[**update_own_profile_api_v1_users_me_patch**](UsersApi.md#update_own_profile_api_v1_users_me_patch) | **PATCH** /api/v1/users/me | Update Own Profile
[**update_user_api_v1_users_user_id_patch**](UsersApi.md#update_user_api_v1_users_user_id_patch) | **PATCH** /api/v1/users/{user_id} | Update User



## create_own_key_api_v1_users_me_keys_post

> models::ApiKeyCreatedOut create_own_key_api_v1_users_me_keys_post(api_key_create_in)
Create Own Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_create_in** | [**ApiKeyCreateIn**](ApiKeyCreateIn.md) |  | [required] |

### Return type

[**models::ApiKeyCreatedOut**](APIKeyCreatedOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_api_v1_users_post

> models::UserOut create_user_api_v1_users_post(user_create_in)
Create User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_create_in** | [**UserCreateIn**](UserCreateIn.md) |  | [required] |

### Return type

[**models::UserOut**](UserOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_key_api_v1_users_user_id_keys_post

> models::ApiKeyCreatedOut create_user_key_api_v1_users_user_id_keys_post(user_id, api_key_create_in)
Create User Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |
**api_key_create_in** | [**ApiKeyCreateIn**](ApiKeyCreateIn.md) |  | [required] |

### Return type

[**models::ApiKeyCreatedOut**](APIKeyCreatedOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_api_v1_users_user_id_delete

> delete_user_api_v1_users_user_id_delete(user_id)
Delete User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_own_profile_api_v1_users_me_get

> models::UserOut get_own_profile_api_v1_users_me_get()
Get Own Profile

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserOut**](UserOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_api_v1_users_user_id_get

> models::UserOut get_user_api_v1_users_user_id_get(user_id)
Get User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |

### Return type

[**models::UserOut**](UserOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_own_keys_api_v1_users_me_keys_get

> Vec<models::ApiKeyOut> list_own_keys_api_v1_users_me_keys_get()
List Own Keys

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiKeyOut>**](APIKeyOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_keys_api_v1_users_user_id_keys_get

> Vec<models::ApiKeyOut> list_user_keys_api_v1_users_user_id_keys_get(user_id)
List User Keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |

### Return type

[**Vec<models::ApiKeyOut>**](APIKeyOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_api_v1_users_get

> models::UserListOut list_users_api_v1_users_get(offset, limit)
List Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**models::UserListOut**](UserListOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_own_key_api_v1_users_me_keys_key_id_delete

> revoke_own_key_api_v1_users_me_keys_key_id_delete(key_id)
Revoke Own Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_key_api_v1_users_user_id_keys_key_id_delete

> revoke_user_key_api_v1_users_user_id_keys_key_id_delete(user_id, key_id)
Revoke User Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |
**key_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_own_profile_api_v1_users_me_patch

> models::UserOut update_own_profile_api_v1_users_me_patch(self_update_in)
Update Own Profile

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**self_update_in** | [**SelfUpdateIn**](SelfUpdateIn.md) |  | [required] |

### Return type

[**models::UserOut**](UserOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_api_v1_users_user_id_patch

> models::UserOut update_user_api_v1_users_user_id_patch(user_id, user_update_in)
Update User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |
**user_update_in** | [**UserUpdateIn**](UserUpdateIn.md) |  | [required] |

### Return type

[**models::UserOut**](UserOut.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

