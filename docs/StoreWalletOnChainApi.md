# \StoreWalletOnChainApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**store_on_chain_wallets_create_on_chain_transaction**](StoreWalletOnChainApi.md#store_on_chain_wallets_create_on_chain_transaction) | **POST** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet/transactions | Create store on-chain wallet transaction
[**store_on_chain_wallets_get_on_chain_fee_rate**](StoreWalletOnChainApi.md#store_on_chain_wallets_get_on_chain_fee_rate) | **GET** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet/feerate | Get store on-chain wallet fee rate
[**store_on_chain_wallets_get_on_chain_wallet_receive_address**](StoreWalletOnChainApi.md#store_on_chain_wallets_get_on_chain_wallet_receive_address) | **GET** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet/address | Get store on-chain wallet address
[**store_on_chain_wallets_get_on_chain_wallet_transaction**](StoreWalletOnChainApi.md#store_on_chain_wallets_get_on_chain_wallet_transaction) | **GET** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet/transactions/{transactionId} | Get store on-chain wallet transaction
[**store_on_chain_wallets_get_on_chain_wallet_utxos**](StoreWalletOnChainApi.md#store_on_chain_wallets_get_on_chain_wallet_utxos) | **GET** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet/utxos | Get store on-chain wallet UTXOS
[**store_on_chain_wallets_patch_on_chain_wallet_transaction**](StoreWalletOnChainApi.md#store_on_chain_wallets_patch_on_chain_wallet_transaction) | **PATCH** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet/transactions/{transactionId} | Patch store on-chain wallet transaction info
[**store_on_chain_wallets_show_on_chain_wallet_overview**](StoreWalletOnChainApi.md#store_on_chain_wallets_show_on_chain_wallet_overview) | **GET** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet | Get store on-chain wallet overview
[**store_on_chain_wallets_show_on_chain_wallet_transactions**](StoreWalletOnChainApi.md#store_on_chain_wallets_show_on_chain_wallet_transactions) | **GET** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet/transactions | Get store on-chain wallet transactions
[**store_on_chain_wallets_un_reserve_on_chain_wallet_receive_address**](StoreWalletOnChainApi.md#store_on_chain_wallets_un_reserve_on_chain_wallet_receive_address) | **DELETE** /api/v1/stores/{storeId}/payment-methods/onchain/{cryptoCode}/wallet/address | UnReserve last store on-chain wallet address



## store_on_chain_wallets_create_on_chain_transaction

> crate::models::StoreOnChainWalletsCreateOnChainTransaction200Response store_on_chain_wallets_create_on_chain_transaction(store_id, crypto_code, create_on_chain_transaction_request)
Create store on-chain wallet transaction

Create store on-chain wallet transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the wallet | [required] |
**create_on_chain_transaction_request** | [**CreateOnChainTransactionRequest**](CreateOnChainTransactionRequest.md) |  | [required] |

### Return type

[**crate::models::StoreOnChainWalletsCreateOnChainTransaction200Response**](StoreOnChainWallets_CreateOnChainTransaction_200_response.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_wallets_get_on_chain_fee_rate

> crate::models::OnChainWalletFeeRateData store_on_chain_wallets_get_on_chain_fee_rate(store_id, crypto_code, block_target)
Get store on-chain wallet fee rate

Get wallet onchain fee rate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |
**block_target** | Option<**f32**> | The number of blocks away you are willing to target for confirmation. Defaults to the wallet's configured `RecommendedFeeBlockTarget` |  |

### Return type

[**crate::models::OnChainWalletFeeRateData**](OnChainWalletFeeRateData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_wallets_get_on_chain_wallet_receive_address

> crate::models::OnChainWalletAddressData store_on_chain_wallets_get_on_chain_wallet_receive_address(store_id, crypto_code, force_generate)
Get store on-chain wallet address

Get or generate address for wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |
**force_generate** | Option<**bool**> | Whether to generate a new address for this request even if the previous one was not used |  |[default to false]

### Return type

[**crate::models::OnChainWalletAddressData**](OnChainWalletAddressData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_wallets_get_on_chain_wallet_transaction

> crate::models::OnChainWalletTransactionData store_on_chain_wallets_get_on_chain_wallet_transaction(store_id, crypto_code, transaction_id)
Get store on-chain wallet transaction

Get store on-chain wallet transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the wallet to fetch | [required] |
**transaction_id** | **String** | The transaction id to fetch | [required] |

### Return type

[**crate::models::OnChainWalletTransactionData**](OnChainWalletTransactionData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_wallets_get_on_chain_wallet_utxos

> Vec<crate::models::OnChainWalletUtxoData> store_on_chain_wallets_get_on_chain_wallet_utxos(store_id, crypto_code)
Get store on-chain wallet UTXOS

Get store on-chain wallet utxos

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the wallet to fetch | [required] |

### Return type

[**Vec<crate::models::OnChainWalletUtxoData>**](OnChainWalletUTXOData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_wallets_patch_on_chain_wallet_transaction

> crate::models::OnChainWalletTransactionData store_on_chain_wallets_patch_on_chain_wallet_transaction(store_id, crypto_code, transaction_id, patch_on_chain_transaction_request, force)
Patch store on-chain wallet transaction info

Patch store on-chain wallet transaction info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the wallet to fetch | [required] |
**transaction_id** | **String** | The transaction id to fetch | [required] |
**patch_on_chain_transaction_request** | [**PatchOnChainTransactionRequest**](PatchOnChainTransactionRequest.md) |  | [required] |
**force** | Option<**String**> | Whether to update the label/comments even if the transaction does not yet exist |  |

### Return type

[**crate::models::OnChainWalletTransactionData**](OnChainWalletTransactionData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_wallets_show_on_chain_wallet_overview

> crate::models::OnChainWalletOverviewData store_on_chain_wallets_show_on_chain_wallet_overview(store_id, crypto_code)
Get store on-chain wallet overview

View information about the specified wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |

### Return type

[**crate::models::OnChainWalletOverviewData**](OnChainWalletOverviewData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_wallets_show_on_chain_wallet_transactions

> Vec<crate::models::OnChainWalletTransactionData> store_on_chain_wallets_show_on_chain_wallet_transactions(store_id, crypto_code, status_filter, label_filter, skip, limit)
Get store on-chain wallet transactions

Get store on-chain wallet transactions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the wallet to fetch | [required] |
**status_filter** | Option<[**Vec<crate::models::TransactionStatus>**](crate::models::TransactionStatus.md)> | Statuses to filter the transactions with |  |
**label_filter** | Option<**String**> | Transaction label to filter by |  |
**skip** | Option<**i32**> | Number of transactions to skip from the start |  |
**limit** | Option<**i32**> | Maximum number of transactions to return |  |

### Return type

[**Vec<crate::models::OnChainWalletTransactionData>**](OnChainWalletTransactionData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## store_on_chain_wallets_un_reserve_on_chain_wallet_receive_address

> store_on_chain_wallets_un_reserve_on_chain_wallet_receive_address(store_id, crypto_code)
UnReserve last store on-chain wallet address

UnReserve address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**crypto_code** | **String** | The crypto code of the payment method to fetch | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

