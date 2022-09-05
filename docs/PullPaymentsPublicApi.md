# \PullPaymentsPublicApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pull_payments_create_payout**](PullPaymentsPublicApi.md#pull_payments_create_payout) | **POST** /api/v1/pull-payments/{pullPaymentId}/payouts | Create Payout
[**pull_payments_get_payout**](PullPaymentsPublicApi.md#pull_payments_get_payout) | **GET** /api/v1/pull-payments/{pullPaymentId}/payouts/{payoutId} | Get Payout
[**pull_payments_get_payouts**](PullPaymentsPublicApi.md#pull_payments_get_payouts) | **GET** /api/v1/pull-payments/{pullPaymentId}/payouts | Get Payouts
[**pull_payments_get_pull_payment**](PullPaymentsPublicApi.md#pull_payments_get_pull_payment) | **GET** /api/v1/pull-payments/{pullPaymentId} | Get Pull Payment



## pull_payments_create_payout

> crate::models::PayoutData pull_payments_create_payout(pull_payment_id, create_payout_request)
Create Payout

Create a new payout

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_payment_id** | **String** | The ID of the pull payment | [required] |
**create_payout_request** | [**CreatePayoutRequest**](CreatePayoutRequest.md) |  | [required] |

### Return type

[**crate::models::PayoutData**](PayoutData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## pull_payments_get_payouts

> Vec<crate::models::PayoutData> pull_payments_get_payouts(pull_payment_id, include_cancelled)
Get Payouts

Get payouts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_payment_id** | **String** | The ID of the pull payment | [required] |
**include_cancelled** | Option<**bool**> | Whether this should list cancelled payouts |  |[default to false]

### Return type

[**Vec<crate::models::PayoutData>**](PayoutData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_payments_get_pull_payment

> crate::models::PullPaymentData pull_payments_get_pull_payment(pull_payment_id)
Get Pull Payment

Get a pull payment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pull_payment_id** | **String** | The ID of the pull payment | [required] |

### Return type

[**crate::models::PullPaymentData**](PullPaymentData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

