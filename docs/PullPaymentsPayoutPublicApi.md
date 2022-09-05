# \PullPaymentsPayoutPublicApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pull_payments_get_payout**](PullPaymentsPayoutPublicApi.md#pull_payments_get_payout) | **GET** /api/v1/pull-payments/{pullPaymentId}/payouts/{payoutId} | Get Payout



## pull_payments_get_payout

> crate::models::PayoutData pull_payments_get_payout(pull_payment_id, payout_id)
Get Payout

Get payout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_payment_id** | **String** | The ID of the pull payment | [required] |
**payout_id** | **String** | The ID of the pull payment payout | [required] |

### Return type

[**crate::models::PayoutData**](PayoutData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

