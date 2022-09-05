# \StorePaymentMethodsLNURLPayApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**store_lnurl_pay_payment_methods_delete_lnurl_pay_payment_method**](StorePaymentMethodsLNURLPayApi.md#store_lnurl_pay_payment_methods_delete_lnurl_pay_payment_method) | **DELETE** /api/v1/stores/{storeId}/payment-methods/LNURL/{cryptoCode} | Remove store LNURL Pay payment method
[**store_lnurl_pay_payment_methods_get_lnurl_pay_payment_method**](StorePaymentMethodsLNURLPayApi.md#store_lnurl_pay_payment_methods_get_lnurl_pay_payment_method) | **GET** /api/v1/stores/{storeId}/payment-methods/LNURL/{cryptoCode} | Get store LNURL Pay payment method
[**store_lnurl_pay_payment_methods_get_lnurl_pay_payment_methods**](StorePaymentMethodsLNURLPayApi.md#store_lnurl_pay_payment_methods_get_lnurl_pay_payment_methods) | **GET** /api/v1/stores/{storeId}/payment-methods/LNURL | Get store LNURL payment methods
[**store_lnurl_pay_payment_methods_update_lnurl_pay_payment_method**](StorePaymentMethodsLNURLPayApi.md#store_lnurl_pay_payment_methods_update_lnurl_pay_payment_method) | **PUT** /api/v1/stores/{storeId}/payment-methods/LNURL/{cryptoCode} | Update store LNURL Pay payment method



## store_lnurl_pay_payment_methods_delete_lnurl_pay_payment_method

> store_lnurl_pay_payment_methods_delete_lnurl_pay_payment_method(store_id, crypto_code)
Remove store LNURL Pay payment method

Removes the specified store payment method.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to update | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lnurl_pay_payment_methods_get_lnurl_pay_payment_method

> crate::models::LnurlPayPaymentMethodData store_lnurl_pay_payment_methods_get_lnurl_pay_payment_method(store_id, crypto_code)
Get store LNURL Pay payment method

View information about the specified payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |

### Return type

[**crate::models::LnurlPayPaymentMethodData**](LNURLPayPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lnurl_pay_payment_methods_get_lnurl_pay_payment_methods

> Vec<crate::models::LnurlPayPaymentMethodData> store_lnurl_pay_payment_methods_get_lnurl_pay_payment_methods(store_id, enabled)
Get store LNURL payment methods

View information about the stores' configured LNURL payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**enabled** | Option<**bool**> | Fetch payment methods that are enabled/disabled only |  |

### Return type

[**Vec<crate::models::LnurlPayPaymentMethodData>**](LNURLPayPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lnurl_pay_payment_methods_update_lnurl_pay_payment_method

> crate::models::LnurlPayPaymentMethodData store_lnurl_pay_payment_methods_update_lnurl_pay_payment_method(store_id, crypto_code, lnurl_pay_payment_method_data)
Update store LNURL Pay payment method

Update the specified store's payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to update | [required] |
**lnurl_pay_payment_method_data** | [**LnurlPayPaymentMethodData**](LnurlPayPaymentMethodData.md) |  | [required] |

### Return type

[**crate::models::LnurlPayPaymentMethodData**](LNURLPayPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

