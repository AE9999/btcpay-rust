# \AuthorizationApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_keys_authorize**](AuthorizationApi.md#api_keys_authorize) | **GET** /api-keys/authorize | Authorize User



## api_keys_authorize

> api_keys_authorize(permissions, application_name, strict, selective_stores, redirect, application_identifier)
Authorize User

Redirect the browser to this endpoint to request the user to generate an api-key with specific permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permissions** | Option<[**Vec<String>**](String.md)> | The permissions to request. (See API Key authentication) |  |
**application_name** | Option<**String**> | The name of your application |  |
**strict** | Option<**bool**> | If permissions are specified, and strict is set to false, it will allow the user to reject some of permissions the application is requesting. |  |[default to true]
**selective_stores** | Option<**bool**> | If the application is requesting the CanModifyStoreSettings permission and selectiveStores is set to true, this allows the user to only grant permissions to selected stores under the user's control. |  |[default to false]
**redirect** | Option<**String**> | The url to redirect to after the user consents, with the query parameters appended to it: permissions, user-id, api-key. If not specified, user is redirected to their API Key list. |  |
**application_identifier** | Option<**String**> | If specified, BTCPay Server will check if there is an existing API key associated with the user that also has this application identifier, redirect host AND the permissions required match(takes selectiveStores and strict into account). `applicationIdentifier` is ignored if redirect is not specified. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

