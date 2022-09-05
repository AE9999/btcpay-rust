# \LightningStoreApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**store_lightning_node_api_connect_to_node**](LightningStoreApi.md#store_lightning_node_api_connect_to_node) | **POST** /api/v1/stores/{storeId}/lightning/{cryptoCode}/connect | Connect to lightning node
[**store_lightning_node_api_create_invoice**](LightningStoreApi.md#store_lightning_node_api_create_invoice) | **POST** /api/v1/stores/{storeId}/lightning/{cryptoCode}/invoices | Create lightning invoice
[**store_lightning_node_api_get_balance**](LightningStoreApi.md#store_lightning_node_api_get_balance) | **GET** /api/v1/stores/{storeId}/lightning/{cryptoCode}/balance | Get node balance
[**store_lightning_node_api_get_channels**](LightningStoreApi.md#store_lightning_node_api_get_channels) | **GET** /api/v1/stores/{storeId}/lightning/{cryptoCode}/channels | Get channels
[**store_lightning_node_api_get_deposit_address**](LightningStoreApi.md#store_lightning_node_api_get_deposit_address) | **POST** /api/v1/stores/{storeId}/lightning/{cryptoCode}/address | Get deposit address
[**store_lightning_node_api_get_info**](LightningStoreApi.md#store_lightning_node_api_get_info) | **GET** /api/v1/stores/{storeId}/lightning/{cryptoCode}/info | Get node information
[**store_lightning_node_api_get_invoice**](LightningStoreApi.md#store_lightning_node_api_get_invoice) | **GET** /api/v1/stores/{storeId}/lightning/{cryptoCode}/invoices/{id} | Get invoice
[**store_lightning_node_api_get_payment**](LightningStoreApi.md#store_lightning_node_api_get_payment) | **GET** /api/v1/stores/{storeId}/lightning/{cryptoCode}/payments/{paymentHash} | Get payment
[**store_lightning_node_api_open_channel**](LightningStoreApi.md#store_lightning_node_api_open_channel) | **POST** /api/v1/stores/{storeId}/lightning/{cryptoCode}/channels | Open channel
[**store_lightning_node_api_pay_invoice**](LightningStoreApi.md#store_lightning_node_api_pay_invoice) | **POST** /api/v1/stores/{storeId}/lightning/{cryptoCode}/invoices/pay | Pay Lightning Invoice



## store_lightning_node_api_connect_to_node

> store_lightning_node_api_connect_to_node(crypto_code, store_id, connect_to_node_request)
Connect to lightning node

Connect to another lightning node.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |
**connect_to_node_request** | [**ConnectToNodeRequest**](ConnectToNodeRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_create_invoice

> crate::models::LightningInvoiceData store_lightning_node_api_create_invoice(crypto_code, store_id, create_lightning_invoice_request)
Create lightning invoice

Create a lightning invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |
**create_lightning_invoice_request** | [**CreateLightningInvoiceRequest**](CreateLightningInvoiceRequest.md) |  | [required] |

### Return type

[**crate::models::LightningInvoiceData**](LightningInvoiceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_get_balance

> crate::models::LightningNodeBalanceData store_lightning_node_api_get_balance(crypto_code, store_id)
Get node balance

View balance of the lightning node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |

### Return type

[**crate::models::LightningNodeBalanceData**](LightningNodeBalanceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_get_channels

> Vec<crate::models::LightningChannelData> store_lightning_node_api_get_channels(crypto_code, store_id)
Get channels

View information about the current channels of the lightning node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |

### Return type

[**Vec<crate::models::LightningChannelData>**](LightningChannelData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_get_deposit_address

> String store_lightning_node_api_get_deposit_address(crypto_code, store_id)
Get deposit address

Get an on-chain deposit address for the lightning node 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |

### Return type

**String**

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_get_info

> crate::models::LightningNodeInformationData store_lightning_node_api_get_info(crypto_code, store_id)
Get node information

View information about the lightning node

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |

### Return type

[**crate::models::LightningNodeInformationData**](LightningNodeInformationData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_get_invoice

> crate::models::LightningInvoiceData store_lightning_node_api_get_invoice(crypto_code, store_id, id)
Get invoice

View information about the requested lightning invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |
**id** | **String** | The id of the lightning invoice. | [required] |

### Return type

[**crate::models::LightningInvoiceData**](LightningInvoiceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_get_payment

> crate::models::LightningPaymentData store_lightning_node_api_get_payment(crypto_code, store_id, payment_hash)
Get payment

View information about the requested lightning payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |
**payment_hash** | **String** | The payment hash of the lightning payment. | [required] |

### Return type

[**crate::models::LightningPaymentData**](LightningPaymentData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_open_channel

> store_lightning_node_api_open_channel(crypto_code, store_id, open_lightning_channel_request)
Open channel

Open a channel with another lightning node. You should connect to that node first.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |
**open_lightning_channel_request** | [**OpenLightningChannelRequest**](OpenLightningChannelRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_lightning_node_api_pay_invoice

> crate::models::LightningPaymentData store_lightning_node_api_pay_invoice(crypto_code, store_id, pay_lightning_invoice_request)
Pay Lightning Invoice

Pay a lightning invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**crypto_code** | **String** | The cryptoCode of the lightning-node to query | [required] |
**store_id** | **String** | The store id with the lightning-node configuration to query | [required] |
**pay_lightning_invoice_request** | [**PayLightningInvoiceRequest**](PayLightningInvoiceRequest.md) |  | [required] |

### Return type

[**crate::models::LightningPaymentData**](LightningPaymentData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

