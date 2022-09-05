# \CustodiansApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**custodians_add_store_custodian_account**](CustodiansApi.md#custodians_add_store_custodian_account) | **POST** /api/v1/stores/{storeId}/custodian-accounts | Add a custodial account to a store.
[**custodians_delete_store_custodian_account**](CustodiansApi.md#custodians_delete_store_custodian_account) | **DELETE** /api/v1/stores/{storeId}/custodian-accounts/{accountId} | Delete store custodian account
[**custodians_get_store_custodian_account**](CustodiansApi.md#custodians_get_store_custodian_account) | **GET** /api/v1/stores/{storeId}/custodian-accounts/{accountId} | Get store custodian account
[**custodians_get_store_custodian_account_deposit_address**](CustodiansApi.md#custodians_get_store_custodian_account_deposit_address) | **GET** /api/v1/stores/{storeId}/custodian-accounts/{accountId}/addresses/{paymentMethod} | Get a deposit address for custodian
[**custodians_get_store_custodian_account_trade_quote**](CustodiansApi.md#custodians_get_store_custodian_account_trade_quote) | **GET** /api/v1/stores/{storeId}/custodian-accounts/{accountId}/trades/quote | Get quote for trading one asset for another
[**custodians_get_store_custodian_account_withdrawal_info**](CustodiansApi.md#custodians_get_store_custodian_account_withdrawal_info) | **POST** /api/v1/stores/{storeId}/custodian-accounts/{accountId}/withdrawals/{withdrawalId} | Get withdrawal info
[**custodians_get_store_custodian_accounts**](CustodiansApi.md#custodians_get_store_custodian_accounts) | **GET** /api/v1/stores/{storeId}/custodian-accounts | List store custodian accounts
[**custodians_get_supported_custodians**](CustodiansApi.md#custodians_get_supported_custodians) | **GET** /api/v1/custodians | List supported custodians
[**custodians_store_custodian_account_trade_market**](CustodiansApi.md#custodians_store_custodian_account_trade_market) | **POST** /api/v1/stores/{storeId}/custodian-accounts/{accountId}/trades/market | Trade one asset for another
[**custodians_update_store_custodian_account**](CustodiansApi.md#custodians_update_store_custodian_account) | **PUT** /api/v1/stores/{storeId}/custodian-accounts/{accountId} | Update custodial account
[**custodians_withdraw_from_store_custodian_account**](CustodiansApi.md#custodians_withdraw_from_store_custodian_account) | **POST** /api/v1/stores/{storeId}/custodian-accounts/{accountId}/withdrawals | Withdraw to store wallet



## custodians_add_store_custodian_account

> crate::models::CustodianAccountData custodians_add_store_custodian_account(store_id, create_custodian_account_request)
Add a custodial account to a store.

Add a custodial account to a store.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**create_custodian_account_request** | [**CreateCustodianAccountRequest**](CreateCustodianAccountRequest.md) |  | [required] |

### Return type

[**crate::models::CustodianAccountData**](CustodianAccountData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_delete_store_custodian_account

> custodians_delete_store_custodian_account(store_id, account_id)
Delete store custodian account

Deletes a custodial account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**account_id** | **String** | The Custodian Account ID | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_get_store_custodian_account

> crate::models::CustodianAccountData custodians_get_store_custodian_account(store_id, account_id, asset_balances)
Get store custodian account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**account_id** | **String** | The Custodian Account ID | [required] |
**asset_balances** | Option<**bool**> | Enable if you want the result to include the 'assetBalances' field. This will make the call slower or could cause the call to fail if the asset balances cannot be loaded (i.e. due to a bad API key). |  |[default to false]

### Return type

[**crate::models::CustodianAccountData**](CustodianAccountData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_get_store_custodian_account_deposit_address

> crate::models::CustodiansGetStoreCustodianAccountDepositAddress200Response custodians_get_store_custodian_account_deposit_address(store_id, account_id, payment_method)
Get a deposit address for custodian

Get a new deposit address for the custodian using the specified payment method (network + crypto code).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**account_id** | **String** | The Custodian Account ID. | [required] |
**payment_method** | **String** | The payment method to use for the deposit. Example: \"BTC-OnChain\" or \"BTC-Lightning\" | [required] |

### Return type

[**crate::models::CustodiansGetStoreCustodianAccountDepositAddress200Response**](Custodians_GetStoreCustodianAccountDepositAddress_200_response.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_get_store_custodian_account_trade_quote

> crate::models::QuoteResultData custodians_get_store_custodian_account_trade_quote(store_id, account_id, from_asset, to_asset)
Get quote for trading one asset for another

Get the current bid and ask price for converting one asset into another.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**account_id** | **String** | The Custodian Account ID. | [required] |
**from_asset** | **String** | The asset to convert. | [required] |
**to_asset** | **String** | The asset you want. | [required] |

### Return type

[**crate::models::QuoteResultData**](QuoteResultData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_get_store_custodian_account_withdrawal_info

> crate::models::WithdrawalResultData custodians_get_store_custodian_account_withdrawal_info(store_id, account_id, withdrawal_id)
Get withdrawal info

Get the details about a past withdrawal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**account_id** | **String** | The Custodian Account ID. | [required] |
**withdrawal_id** | **String** | The Withdrawal ID. | [required] |

### Return type

[**crate::models::WithdrawalResultData**](WithdrawalResultData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_get_store_custodian_accounts

> Vec<crate::models::CustodianAccountData> custodians_get_store_custodian_accounts(store_id, asset_balances)
List store custodian accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**asset_balances** | Option<**bool**> | Enable if you want the result to include the 'assetBalances' field. This will make the call slower or could cause the call to fail if the asset balances cannot be loaded (i.e. due to a bad API key). |  |[default to false]

### Return type

[**Vec<crate::models::CustodianAccountData>**](CustodianAccountData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_get_supported_custodians

> Vec<crate::models::CustodianData> custodians_get_supported_custodians()
List supported custodians

List all supported custodians for the BTCPay instance. You can install plugins to add more custodians.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::CustodianData>**](CustodianData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_store_custodian_account_trade_market

> crate::models::TradeResultData custodians_store_custodian_account_trade_market(store_id, account_id, trade_request_data)
Trade one asset for another

Trade one asset for another using a market order (=instant purchase with instant result or failure). A suitable asset pair will automatically be selected. If no asset pair is available, the call will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**account_id** | **String** | The Custodian Account ID. | [required] |
**trade_request_data** | Option<[**TradeRequestData**](TradeRequestData.md)> |  |  |

### Return type

[**crate::models::TradeResultData**](TradeResultData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_update_store_custodian_account

> crate::models::CustodianAccountData custodians_update_store_custodian_account(store_id, account_id, create_custodian_account_request)
Update custodial account

Update custodial account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**account_id** | **String** | The Custodian Account ID | [required] |
**create_custodian_account_request** | [**CreateCustodianAccountRequest**](CreateCustodianAccountRequest.md) |  | [required] |

### Return type

[**crate::models::CustodianAccountData**](CustodianAccountData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custodians_withdraw_from_store_custodian_account

> crate::models::WithdrawalResultData custodians_withdraw_from_store_custodian_account(store_id, account_id, withdrawal_request_data)
Withdraw to store wallet

Withdraw an asset to your store wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The Store ID | [required] |
**account_id** | **String** | The Custodian Account ID. | [required] |
**withdrawal_request_data** | [**WithdrawalRequestData**](WithdrawalRequestData.md) |  | [required] |

### Return type

[**crate::models::WithdrawalResultData**](WithdrawalResultData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

