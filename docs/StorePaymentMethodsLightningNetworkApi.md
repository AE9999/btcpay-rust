# \StorePaymentMethodsLightningNetworkApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**store_lightning_network_payment_methods_delete_lightning_network_payment_method**](StorePaymentMethodsLightningNetworkApi.md#store_lightning_network_payment_methods_delete_lightning_network_payment_method) | **DELETE** /api/v1/stores/{storeId}/payment-methods/LightningNetwork/{cryptoCode} | Remove store Lightning Network payment method
[**store_lightning_network_payment_methods_get_lightning_network_payment_method**](StorePaymentMethodsLightningNetworkApi.md#store_lightning_network_payment_methods_get_lightning_network_payment_method) | **GET** /api/v1/stores/{storeId}/payment-methods/LightningNetwork/{cryptoCode} | Get store Lightning Network payment method
[**store_lightning_network_payment_methods_get_lightning_network_payment_methods**](StorePaymentMethodsLightningNetworkApi.md#store_lightning_network_payment_methods_get_lightning_network_payment_methods) | **GET** /api/v1/stores/{storeId}/payment-methods/LightningNetwork | Get store Lightning Network payment methods
[**store_lightning_network_payment_methods_update_lightning_network_payment_method**](StorePaymentMethodsLightningNetworkApi.md#store_lightning_network_payment_methods_update_lightning_network_payment_method) | **PUT** /api/v1/stores/{storeId}/payment-methods/LightningNetwork/{cryptoCode} | Update store Lightning Network payment method



## store_lightning_network_payment_methods_delete_lightning_network_payment_method

> store_lightning_network_payment_methods_delete_lightning_network_payment_method(store_id, crypto_code)
Remove store Lightning Network payment method

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


## store_lightning_network_payment_methods_get_lightning_network_payment_method

> crate::models::LightningNetworkPaymentMethodData store_lightning_network_payment_methods_get_lightning_network_payment_method(store_id, crypto_code)
Get store Lightning Network payment method

View information about the specified payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |

### Return type

[**crate::models::LightningNetworkPaymentMethodData**](LightningNetworkPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_network_payment_methods_get_lightning_network_payment_methods

> Vec<crate::models::LightningNetworkPaymentMethodData> store_lightning_network_payment_methods_get_lightning_network_payment_methods(store_id, enabled)
Get store Lightning Network payment methods

View information about the stores' configured Lightning Network payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**enabled** | Option<**bool**> | Fetch payment methods that are enabled/disabled only |  |

### Return type

[**Vec<crate::models::LightningNetworkPaymentMethodData>**](LightningNetworkPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_network_payment_methods_update_lightning_network_payment_method

> crate::models::LightningNetworkPaymentMethodData store_lightning_network_payment_methods_update_lightning_network_payment_method(store_id, crypto_code, update_lightning_network_payment_method_request)
Update store Lightning Network payment method

Update the specified store's payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to update | [required] |
**update_lightning_network_payment_method_request** | [**UpdateLightningNetworkPaymentMethodRequest**](UpdateLightningNetworkPaymentMethodRequest.md) |  | [required] |

### Return type

[**crate::models::LightningNetworkPaymentMethodData**](LightningNetworkPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

