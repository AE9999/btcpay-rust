# \StoresPayoutsApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payouts_create_payout_through_store**](StoresPayoutsApi.md#payouts_create_payout_through_store) | **POST** /api/v1/stores/{storeId}/payouts | Create Payout 
[**pull_payments_approve_payout**](StoresPayoutsApi.md#pull_payments_approve_payout) | **POST** /api/v1/stores/{storeId}/payouts/{payoutId} | Approve Payout
[**pull_payments_cancel_payout**](StoresPayoutsApi.md#pull_payments_cancel_payout) | **DELETE** /api/v1/stores/{storeId}/payouts/{payoutId} | Cancel Payout
[**pull_payments_get_store_payouts**](StoresPayoutsApi.md#pull_payments_get_store_payouts) | **GET** /api/v1/stores/{storeId}/payouts | Get Store Payouts
[**pull_payments_mark_payout_paid**](StoresPayoutsApi.md#pull_payments_mark_payout_paid) | **POST** /api/v1/stores/{storeId}/payouts/{payoutId}/mark-paid | Mark Payout as Paid



## payouts_create_payout_through_store

> crate::models::PayoutData payouts_create_payout_through_store(store_id, create_payout_through_store_request)
Create Payout 

Create a new payout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The ID of the store | [required] |
**create_payout_through_store_request** | [**CreatePayoutThroughStoreRequest**](CreatePayoutThroughStoreRequest.md) |  | [required] |

### Return type

[**crate::models::PayoutData**](PayoutData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_payments_approve_payout

> crate::models::PayoutData pull_payments_approve_payout(store_id, payout_id, pull_payments_approve_payout_request)
Approve Payout

Approve a payout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The ID of the store | [required] |
**payout_id** | **String** | The ID of the payout | [required] |
**pull_payments_approve_payout_request** | Option<[**PullPaymentsApprovePayoutRequest**](PullPaymentsApprovePayoutRequest.md)> |  |  |

### Return type

[**crate::models::PayoutData**](PayoutData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_payments_cancel_payout

> pull_payments_cancel_payout(store_id, payout_id)
Cancel Payout

Cancel the payout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The ID of the store | [required] |
**payout_id** | **String** | The ID of the payout | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_payments_get_store_payouts

> Vec<crate::models::PayoutData> pull_payments_get_store_payouts(store_id, include_cancelled)
Get Store Payouts

Get payouts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The ID of the store | [required] |
**include_cancelled** | Option<**bool**> | Whether this should list cancelled payouts |  |[default to false]

### Return type

[**Vec<crate::models::PayoutData>**](PayoutData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_payments_mark_payout_paid

> pull_payments_mark_payout_paid(store_id, payout_id)
Mark Payout as Paid

Mark a payout as paid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The ID of the store | [required] |
**payout_id** | **String** | The ID of the payout | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

