# \PaymentRequestsApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**payment_requests_archive_payment_request**](PaymentRequestsApi.md#payment_requests_archive_payment_request) | **DELETE** /api/v1/stores/{storeId}/payment-requests/{paymentRequestId} | Archive payment request
[**payment_requests_create_payment_request**](PaymentRequestsApi.md#payment_requests_create_payment_request) | **POST** /api/v1/stores/{storeId}/payment-requests | Create a new payment request
[**payment_requests_get_payment_request**](PaymentRequestsApi.md#payment_requests_get_payment_request) | **GET** /api/v1/stores/{storeId}/payment-requests/{paymentRequestId} | Get payment request
[**payment_requests_get_payment_requests**](PaymentRequestsApi.md#payment_requests_get_payment_requests) | **GET** /api/v1/stores/{storeId}/payment-requests | Get payment requests
[**payment_requests_update_payment_request**](PaymentRequestsApi.md#payment_requests_update_payment_request) | **PUT** /api/v1/stores/{storeId}/payment-requests/{paymentRequestId} | Update payment request



## payment_requests_archive_payment_request

> payment_requests_archive_payment_request(store_id, payment_request_id)
Archive payment request

Archives the specified payment request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store the payment request belongs to | [required] |
**payment_request_id** | **String** | The payment request to remove | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_requests_create_payment_request

> crate::models::PaymentRequestData payment_requests_create_payment_request(store_id, payment_request_base_data)
Create a new payment request

Create a new payment request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to query | [required] |
**payment_request_base_data** | [**PaymentRequestBaseData**](PaymentRequestBaseData.md) |  | [required] |

### Return type

[**crate::models::PaymentRequestData**](PaymentRequestData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_requests_get_payment_request

> crate::models::PaymentRequestData payment_requests_get_payment_request(store_id, payment_request_id)
Get payment request

View information about the specified payment request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**payment_request_id** | **String** | The payment request to fetch | [required] |

### Return type

[**crate::models::PaymentRequestData**](PaymentRequestData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_requests_get_payment_requests

> Vec<crate::models::PaymentRequestData> payment_requests_get_payment_requests(store_id)
Get payment requests

View information about the existing payment requests

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to query | [required] |

### Return type

[**Vec<crate::models::PaymentRequestData>**](PaymentRequestData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_requests_update_payment_request

> crate::models::PaymentRequestData payment_requests_update_payment_request(store_id, payment_request_id, payment_request_base_data)
Update payment request

Update a payment request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to query | [required] |
**payment_request_id** | **String** | The payment request to update | [required] |
**payment_request_base_data** | [**PaymentRequestBaseData**](PaymentRequestBaseData.md) |  | [required] |

### Return type

[**crate::models::PaymentRequestData**](PaymentRequestData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

