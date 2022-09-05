# \PullPaymentsManagementApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pull_payments_archive_pull_payment**](PullPaymentsManagementApi.md#pull_payments_archive_pull_payment) | **DELETE** /api/v1/stores/{storeId}/pull-payments/{pullPaymentId} | Archive a pull payment
[**pull_payments_create_pull_payment**](PullPaymentsManagementApi.md#pull_payments_create_pull_payment) | **POST** /api/v1/stores/{storeId}/pull-payments | Create a new pull payment
[**pull_payments_get_pull_payments**](PullPaymentsManagementApi.md#pull_payments_get_pull_payments) | **GET** /api/v1/stores/{storeId}/pull-payments | Get store's pull payments



## pull_payments_archive_pull_payment

> pull_payments_archive_pull_payment(store_id, pull_payment_id)
Archive a pull payment

Archive this pull payment (Will cancel all payouts awaiting for payment)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The ID of the store | [required] |
**pull_payment_id** | **String** | The ID of the pull payment | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_payments_create_pull_payment

> crate::models::PullPaymentData pull_payments_create_pull_payment(store_id, pull_payments_create_pull_payment_request)
Create a new pull payment

A pull payment allows its receiver to ask for payouts up to `amount` of `currency` every `period`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store ID | [required] |
**pull_payments_create_pull_payment_request** | Option<[**PullPaymentsCreatePullPaymentRequest**](PullPaymentsCreatePullPaymentRequest.md)> |  |  |

### Return type

[**crate::models::PullPaymentData**](PullPaymentData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pull_payments_get_pull_payments

> Vec<crate::models::PullPaymentData> pull_payments_get_pull_payments(store_id, include_archived)
Get store's pull payments

Get the pull payments of a store

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store ID | [required] |
**include_archived** | Option<**bool**> | Whether this should list archived pull payments |  |[default to false]

### Return type

[**Vec<crate::models::PullPaymentData>**](PullPaymentData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

