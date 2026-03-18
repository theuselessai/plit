# \SettingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_settings_api_v1_settings_get**](SettingsApi.md#get_settings_api_v1_settings_get) | **GET** /api/v1/settings/ | Get Settings
[**recheck_environment_api_v1_settings_recheck_environment_post**](SettingsApi.md#recheck_environment_api_v1_settings_recheck_environment_post) | **POST** /api/v1/settings/recheck-environment/ | Recheck Environment
[**update_settings_api_v1_settings_patch**](SettingsApi.md#update_settings_api_v1_settings_patch) | **PATCH** /api/v1/settings/ | Update Settings



## get_settings_api_v1_settings_get

> models::SettingsResponse get_settings_api_v1_settings_get()
Get Settings

Return current platform config + cached environment info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SettingsResponse**](SettingsResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## recheck_environment_api_v1_settings_recheck_environment_post

> std::collections::HashMap<String, serde_json::Value> recheck_environment_api_v1_settings_recheck_environment_post()
Recheck Environment

Force re-detection of environment capabilities.

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_settings_api_v1_settings_patch

> models::SettingsUpdateResponse update_settings_api_v1_settings_patch(settings_update)
Update Settings

Update conf.json fields. Hot-reloads applicable settings in-process.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**settings_update** | [**SettingsUpdate**](SettingsUpdate.md) |  | [required] |

### Return type

[**models::SettingsUpdateResponse**](SettingsUpdateResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

