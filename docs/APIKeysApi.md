# \APIKeysApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_keys_create_api_key**](APIKeysApi.md#api_keys_create_api_key) | **POST** /api/v1/api-keys | Create a new API Key
[**api_keys_delete_api_key**](APIKeysApi.md#api_keys_delete_api_key) | **DELETE** /api/v1/api-keys/{apikey} | Revoke an API Key
[**api_keys_delete_current_api_key**](APIKeysApi.md#api_keys_delete_current_api_key) | **DELETE** /api/v1/api-keys/current | Revoke the current API Key
[**api_keys_get_current_api_key**](APIKeysApi.md#api_keys_get_current_api_key) | **GET** /api/v1/api-keys/current | Get the current API Key information



## api_keys_create_api_key

> crate::models::ApiKeyData api_keys_create_api_key(api_keys_create_api_key_request)
Create a new API Key

Create a new API Key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_keys_create_api_key_request** | Option<[**ApiKeysCreateApiKeyRequest**](ApiKeysCreateApiKeyRequest.md)> |  |  |

### Return type

[**crate::models::ApiKeyData**](ApiKeyData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_keys_delete_api_key

> api_keys_delete_api_key(apikey)
Revoke an API Key

Revoke the current API key so that it cannot be used anymore

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | The API Key to revoke | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_keys_delete_current_api_key

> crate::models::ApiKeyData api_keys_delete_current_api_key()
Revoke the current API Key

Revoke the current API key so that it cannot be used anymore

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiKeyData**](ApiKeyData.md)

### Authorization

[API_Key](../README.md#API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_keys_get_current_api_key

> crate::models::ApiKeyData api_keys_get_current_api_key()
Get the current API Key information

View information about the current API key

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ApiKeyData**](ApiKeyData.md)

### Authorization

[API_Key](../README.md#API_Key)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

