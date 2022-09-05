# \StorePaymentMethodsApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**store_payment_methods_get_store_payment_methods**](StorePaymentMethodsApi.md#store_payment_methods_get_store_payment_methods) | **GET** /api/v1/stores/{storeId}/payment-methods | Get store payment methods



## store_payment_methods_get_store_payment_methods

> Vec<crate::models::GenericPaymentMethodData> store_payment_methods_get_store_payment_methods(store_id, enabled)
Get store payment methods

View information about the stores' configured payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**enabled** | Option<**bool**> | Fetch payment methods that are enabled/disabled only |  |

### Return type

[**Vec<crate::models::GenericPaymentMethodData>**](GenericPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

