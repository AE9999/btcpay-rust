# \StoresEmailApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stores_get_store_email_settings**](StoresEmailApi.md#stores_get_store_email_settings) | **GET** /api/v1/stores/{storeId}/email | Get store email settings
[**stores_send_store_email**](StoresEmailApi.md#stores_send_store_email) | **POST** /api/v1/stores/{storeId}/email/send | Send an email for a store
[**stores_update_store_email_settings**](StoresEmailApi.md#stores_update_store_email_settings) | **PUT** /api/v1/stores/{storeId}/email | Update store email settings



## stores_get_store_email_settings

> crate::models::EmailSettingsData stores_get_store_email_settings(store_id)
Get store email settings

View email settings of the specified store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |

### Return type

[**crate::models::EmailSettingsData**](EmailSettingsData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stores_send_store_email

> stores_send_store_email(store_id, email_data)
Send an email for a store

Send an email using the store's SMTP server

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to send the email from | [required] |
**email_data** | [**EmailData**](EmailData.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stores_update_store_email_settings

> crate::models::EmailSettingsData stores_update_store_email_settings(store_id, email_settings_data)
Update store email settings

Update a store's email settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to update | [required] |
**email_settings_data** | [**EmailSettingsData**](EmailSettingsData.md) |  | [required] |

### Return type

[**crate::models::EmailSettingsData**](EmailSettingsData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

