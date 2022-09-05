# \StorePaymentMethodsOnChainApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**store_on_chain_payment_methods_delete_on_chain_payment_method**](StorePaymentMethodsOnChainApi.md#store_on_chain_payment_methods_delete_on_chain_payment_method) | **DELETE** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode} | Remove store on-chain payment method
[**store_on_chain_payment_methods_generate_on_chain_wallet**](StorePaymentMethodsOnChainApi.md#store_on_chain_payment_methods_generate_on_chain_wallet) | **POST** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/generate | Generate store on-chain wallet
[**store_on_chain_payment_methods_get_on_chain_payment_method**](StorePaymentMethodsOnChainApi.md#store_on_chain_payment_methods_get_on_chain_payment_method) | **GET** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode} | Get store on-chain payment method
[**store_on_chain_payment_methods_get_on_chain_payment_method_preview**](StorePaymentMethodsOnChainApi.md#store_on_chain_payment_methods_get_on_chain_payment_method_preview) | **GET** /api/v1/stores/{storeId}/payment-methods/OnChain/{cryptoCode}/preview | Preview store on-chain payment method addresses
[**store_on_chain_payment_methods_get_on_chain_payment_methods**](StorePaymentMethodsOnChainApi.md#store_on_chain_payment_methods_get_on_chain_payment_methods) | **GET** /api/v1/stores/{storeId}/payment-methods/OnChain | Get store on-chain payment methods
[**store_on_chain_payment_methods_poston_chain_payment_method_preview**](StorePaymentMethodsOnChainApi.md#store_on_chain_payment_methods_poston_chain_payment_method_preview) | **POST** /api/v1/stores/{storeId}/payment-methods/OnChain/{cryptoCode}/preview | Preview proposed store on-chain payment method addresses
[**store_on_chain_payment_methods_update_on_chain_payment_method**](StorePaymentMethodsOnChainApi.md#store_on_chain_payment_methods_update_on_chain_payment_method) | **PUT** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode} | Update store on-chain payment method



## store_on_chain_payment_methods_delete_on_chain_payment_method

> store_on_chain_payment_methods_delete_on_chain_payment_method(store_id, crypto_code)
Remove store on-chain payment method

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


## store_on_chain_payment_methods_generate_on_chain_wallet

> crate::models::OnChainPaymentMethodDataWithSensitiveData store_on_chain_payment_methods_generate_on_chain_wallet(store_id, crypto_code, generate_on_chain_wallet_request)
Generate store on-chain wallet

Generate a wallet and update the specified store's payment method to it

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to update | [required] |
**generate_on_chain_wallet_request** | [**GenerateOnChainWalletRequest**](GenerateOnChainWalletRequest.md) |  | [required] |

### Return type

[**crate::models::OnChainPaymentMethodDataWithSensitiveData**](OnChainPaymentMethodDataWithSensitiveData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_payment_methods_get_on_chain_payment_method

> crate::models::OnChainPaymentMethodData store_on_chain_payment_methods_get_on_chain_payment_method(store_id, crypto_code)
Get store on-chain payment method

View information about the specified payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |

### Return type

[**crate::models::OnChainPaymentMethodData**](OnChainPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_payment_methods_get_on_chain_payment_method_preview

> crate::models::OnChainPaymentMethodPreviewResultData store_on_chain_payment_methods_get_on_chain_payment_method_preview(store_id, crypto_code, offset, amount)
Preview store on-chain payment method addresses

View addresses of the current payment method of the store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |
**offset** | Option<**f32**> | From which index to fetch the addresses |  |
**amount** | Option<**f32**> | Number of addresses to preview |  |

### Return type

[**crate::models::OnChainPaymentMethodPreviewResultData**](OnChainPaymentMethodPreviewResultData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_payment_methods_get_on_chain_payment_methods

> Vec<crate::models::OnChainPaymentMethodData> store_on_chain_payment_methods_get_on_chain_payment_methods(store_id, enabled)
Get store on-chain payment methods

View information about the stores' configured on-chain payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**enabled** | Option<**bool**> | Fetch payment methods that are enabled/disabled only |  |

### Return type

[**Vec<crate::models::OnChainPaymentMethodData>**](OnChainPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_payment_methods_poston_chain_payment_method_preview

> crate::models::OnChainPaymentMethodPreviewResultData store_on_chain_payment_methods_poston_chain_payment_method_preview(store_id, crypto_code, store_on_chain_payment_methods_poston_chain_payment_method_preview_request, offset, amount)
Preview proposed store on-chain payment method addresses

View addresses of a proposed payment method of the store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |
**store_on_chain_payment_methods_poston_chain_payment_method_preview_request** | [**StoreOnChainPaymentMethodsPostonChainPaymentMethodPreviewRequest**](StoreOnChainPaymentMethodsPostonChainPaymentMethodPreviewRequest.md) |  | [required] |
**offset** | Option<**f32**> | From which index to fetch the addresses |  |
**amount** | Option<**f32**> | Number of addresses to preview |  |

### Return type

[**crate::models::OnChainPaymentMethodPreviewResultData**](OnChainPaymentMethodPreviewResultData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_payment_methods_update_on_chain_payment_method

> crate::models::OnChainPaymentMethodData store_on_chain_payment_methods_update_on_chain_payment_method(store_id, crypto_code, update_on_chain_payment_method_request)
Update store on-chain payment method

Update the specified store's payment method

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to update | [required] |
**update_on_chain_payment_method_request** | [**UpdateOnChainPaymentMethodRequest**](UpdateOnChainPaymentMethodRequest.md) |  | [required] |

### Return type

[**crate::models::OnChainPaymentMethodData**](OnChainPaymentMethodData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

