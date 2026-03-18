# \AuthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**me_api_v1_auth_me_get**](AuthApi.md#me_api_v1_auth_me_get) | **GET** /api/v1/auth/me/ | Me
[**mfa_disable_api_v1_auth_mfa_disable_post**](AuthApi.md#mfa_disable_api_v1_auth_mfa_disable_post) | **POST** /api/v1/auth/mfa/disable/ | Mfa Disable
[**mfa_login_verify_api_v1_auth_mfa_login_verify_post**](AuthApi.md#mfa_login_verify_api_v1_auth_mfa_login_verify_post) | **POST** /api/v1/auth/mfa/login-verify/ | Mfa Login Verify
[**mfa_reset_api_v1_auth_mfa_reset_post**](AuthApi.md#mfa_reset_api_v1_auth_mfa_reset_post) | **POST** /api/v1/auth/mfa/reset/ | Mfa Reset
[**mfa_setup_api_v1_auth_mfa_setup_post**](AuthApi.md#mfa_setup_api_v1_auth_mfa_setup_post) | **POST** /api/v1/auth/mfa/setup/ | Mfa Setup
[**mfa_status_api_v1_auth_mfa_status_get**](AuthApi.md#mfa_status_api_v1_auth_mfa_status_get) | **GET** /api/v1/auth/mfa/status/ | Mfa Status
[**mfa_verify_api_v1_auth_mfa_verify_post**](AuthApi.md#mfa_verify_api_v1_auth_mfa_verify_post) | **POST** /api/v1/auth/mfa/verify/ | Mfa Verify
[**obtain_token_api_v1_auth_token_post**](AuthApi.md#obtain_token_api_v1_auth_token_post) | **POST** /api/v1/auth/token/ | Obtain Token



## me_api_v1_auth_me_get

> models::MeResponse me_api_v1_auth_me_get()
Me

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MeResponse**](MeResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mfa_disable_api_v1_auth_mfa_disable_post

> models::MfaStatusResponse mfa_disable_api_v1_auth_mfa_disable_post(mfa_disable_request)
Mfa Disable

Disable MFA after verifying a TOTP code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_disable_request** | [**MfaDisableRequest**](MfaDisableRequest.md) |  | [required] |

### Return type

[**models::MfaStatusResponse**](MFAStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mfa_login_verify_api_v1_auth_mfa_login_verify_post

> models::TokenResponse mfa_login_verify_api_v1_auth_mfa_login_verify_post(mfa_login_verify_request)
Mfa Login Verify

Complete MFA login — accepts username + TOTP code, issues API key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_login_verify_request** | [**MfaLoginVerifyRequest**](MfaLoginVerifyRequest.md) |  | [required] |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mfa_reset_api_v1_auth_mfa_reset_post

> models::MfaStatusResponse mfa_reset_api_v1_auth_mfa_reset_post()
Mfa Reset

Emergency MFA reset — only allowed from loopback addresses.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MfaStatusResponse**](MFAStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mfa_setup_api_v1_auth_mfa_setup_post

> models::MfaSetupResponse mfa_setup_api_v1_auth_mfa_setup_post()
Mfa Setup

Generate a TOTP secret. Does NOT enable MFA until /mfa/verify/ is called.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MfaSetupResponse**](MFASetupResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mfa_status_api_v1_auth_mfa_status_get

> models::MfaStatusResponse mfa_status_api_v1_auth_mfa_status_get()
Mfa Status

Return current MFA status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::MfaStatusResponse**](MFAStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mfa_verify_api_v1_auth_mfa_verify_post

> models::MfaStatusResponse mfa_verify_api_v1_auth_mfa_verify_post(mfa_verify_request)
Mfa Verify

Verify a TOTP code and enable MFA on the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mfa_verify_request** | [**MfaVerifyRequest**](MfaVerifyRequest.md) |  | [required] |

### Return type

[**models::MfaStatusResponse**](MFAStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## obtain_token_api_v1_auth_token_post

> models::TokenResponse obtain_token_api_v1_auth_token_post(token_request)
Obtain Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token_request** | [**TokenRequest**](TokenRequest.md) |  | [required] |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

