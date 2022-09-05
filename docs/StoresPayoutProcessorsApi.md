# \StoresPayoutProcessorsApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_payout_processors_for_payment_method**](StoresPayoutProcessorsApi.md#greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_payout_processors_for_payment_method) | **GET** /api/v1/stores/{storeId}/payout-processors/LightningAutomatedTransferSenderFactory/{paymentMethod} | Get configured store Lightning automated payout processors
[**greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_transfer_sender_factory**](StoresPayoutProcessorsApi.md#greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_transfer_sender_factory) | **GET** /api/v1/stores/{storeId}/payout-processors/LightningAutomatedTransferSenderFactory | Get configured store Lightning automated payout processors
[**greenfield_store_automated_lightning_payout_processors_controller_update_store_lightning_automated_payout_processor**](StoresPayoutProcessorsApi.md#greenfield_store_automated_lightning_payout_processors_controller_update_store_lightning_automated_payout_processor) | **PUT** /api/v1/stores/{storeId}/payout-processors/LightningAutomatedTransferSenderFactory/{paymentMethod} | Update configured store Lightning automated payout processors
[**greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_payout_processors_for_payment_method**](StoresPayoutProcessorsApi.md#greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_payout_processors_for_payment_method) | **GET** /api/v1/stores/{storeId}/payout-processors/OnChainAutomatedTransferSenderFactory/{paymentMethod} | Get configured store onchain automated payout processors
[**greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_transfer_sender_factory**](StoresPayoutProcessorsApi.md#greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_transfer_sender_factory) | **GET** /api/v1/stores/{storeId}/payout-processors/OnChainAutomatedTransferSenderFactory | Get configured store onchain automated payout processors
[**greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_payout_processor_for_payment_method**](StoresPayoutProcessorsApi.md#greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_payout_processor_for_payment_method) | **PUT** /api/v1/stores/{storeId}/payout-processors/OnChainAutomatedTransferSenderFactory/{paymentMethod} | Update configured store onchain automated payout processors
[**greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_transfer_sender_factory**](StoresPayoutProcessorsApi.md#greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_transfer_sender_factory) | **PUT** /api/v1/stores/{storeId}/payout-processors/OnChainAutomatedTransferSenderFactory | Update configured store onchain automated payout processors
[**store_payout_processors_get_store_payout_processors**](StoresPayoutProcessorsApi.md#store_payout_processors_get_store_payout_processors) | **GET** /api/v1/stores/{storeId}/payout-processors | Get store configured payout processors
[**store_payout_processors_remove_store_payout_processor**](StoresPayoutProcessorsApi.md#store_payout_processors_remove_store_payout_processor) | **DELETE** /api/v1/stores/{storeId}/payout-processors/{processor}/{paymentMethod} | Remove store configured payout processor



## greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_payout_processors_for_payment_method

> Vec<crate::models::LightningAutomatedTransferSettings> greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_payout_processors_for_payment_method(store_id, payment_method)
Get configured store Lightning automated payout processors

Get configured store Lightning automated payout processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**payment_method** | **String** | A specific payment method to fetch | [required] |

### Return type

[**Vec<crate::models::LightningAutomatedTransferSettings>**](LightningAutomatedTransferSettings.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_transfer_sender_factory

> Vec<crate::models::LightningAutomatedTransferSettings> greenfield_store_automated_lightning_payout_processors_controller_get_store_lightning_automated_transfer_sender_factory(store_id)
Get configured store Lightning automated payout processors

Get configured store Lightning automated payout processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |

### Return type

[**Vec<crate::models::LightningAutomatedTransferSettings>**](LightningAutomatedTransferSettings.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## greenfield_store_automated_lightning_payout_processors_controller_update_store_lightning_automated_payout_processor

> crate::models::LightningAutomatedTransferSettings greenfield_store_automated_lightning_payout_processors_controller_update_store_lightning_automated_payout_processor(store_id, payment_method, update_lightning_automated_transfer_settings)
Update configured store Lightning automated payout processors

Update configured store Lightning automated payout processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**payment_method** | **String** | A specific payment method to fetch | [required] |
**update_lightning_automated_transfer_settings** | [**UpdateLightningAutomatedTransferSettings**](UpdateLightningAutomatedTransferSettings.md) |  | [required] |

### Return type

[**crate::models::LightningAutomatedTransferSettings**](LightningAutomatedTransferSettings.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_payout_processors_for_payment_method

> Vec<crate::models::OnChainAutomatedTransferSettings> greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_payout_processors_for_payment_method(store_id, payment_method)
Get configured store onchain automated payout processors

Get configured store onchain automated payout processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**payment_method** | **String** | A specific payment method to fetch | [required] |

### Return type

[**Vec<crate::models::OnChainAutomatedTransferSettings>**](OnChainAutomatedTransferSettings.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_transfer_sender_factory

> Vec<crate::models::OnChainAutomatedTransferSettings> greenfield_store_automated_on_chain_payout_processors_controller_get_store_on_chain_automated_transfer_sender_factory(store_id)
Get configured store onchain automated payout processors

Get configured store onchain automated payout processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |

### Return type

[**Vec<crate::models::OnChainAutomatedTransferSettings>**](OnChainAutomatedTransferSettings.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_payout_processor_for_payment_method

> crate::models::OnChainAutomatedTransferSettings greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_payout_processor_for_payment_method(store_id, payment_method, update_on_chain_automated_transfer_settings)
Update configured store onchain automated payout processors

Update configured store onchain automated payout processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**payment_method** | **String** | A specific payment method to fetch | [required] |
**update_on_chain_automated_transfer_settings** | [**UpdateOnChainAutomatedTransferSettings**](UpdateOnChainAutomatedTransferSettings.md) |  | [required] |

### Return type

[**crate::models::OnChainAutomatedTransferSettings**](OnChainAutomatedTransferSettings.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_transfer_sender_factory

> crate::models::OnChainAutomatedTransferSettings greenfield_store_automated_on_chain_payout_processors_controller_update_store_on_chain_automated_transfer_sender_factory(store_id, update_on_chain_automated_transfer_settings)
Update configured store onchain automated payout processors

Update configured store onchain automated payout processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**update_on_chain_automated_transfer_settings** | [**UpdateOnChainAutomatedTransferSettings**](UpdateOnChainAutomatedTransferSettings.md) |  | [required] |

### Return type

[**crate::models::OnChainAutomatedTransferSettings**](OnChainAutomatedTransferSettings.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_payout_processors_get_store_payout_processors

> Vec<crate::models::PayoutProcessorData> store_payout_processors_get_store_payout_processors(store_id)
Get store configured payout processors

Get store configured payout processors

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |

### Return type

[**Vec<crate::models::PayoutProcessorData>**](PayoutProcessorData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_payout_processors_remove_store_payout_processor

> store_payout_processors_remove_store_payout_processor(store_id, processor, payment_method)
Remove store configured payout processor

Remove store configured payout processor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store | [required] |
**processor** | **String** | The processor | [required] |
**payment_method** | **String** | The payment method | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

