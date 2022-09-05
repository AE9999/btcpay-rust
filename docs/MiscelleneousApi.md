# \MiscelleneousApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invoice_checkout**](MiscelleneousApi.md#invoice_checkout) | **GET** /i/{invoiceId} | Invoice checkout
[**lang_codes**](MiscelleneousApi.md#lang_codes) | **GET** /misc/lang | Language codes
[**permissions_metadata**](MiscelleneousApi.md#permissions_metadata) | **GET** /misc/permissions | Permissions metadata



## invoice_checkout

> invoice_checkout(invoice_id, lang)
Invoice checkout

View the checkout page of an invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invoice_id** | **String** | The invoice id | [required] |
**lang** | Option<**String**> | The preferred language of the checkout page. You can use \"auto\" to use the language of the customer's browser or see the list of language codes with [this operation](#operation/langCodes). |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lang_codes

> Vec<crate::models::LangCodes200ResponseInner> lang_codes()
Language codes

The supported language codes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::LangCodes200ResponseInner>**](langCodes_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## permissions_metadata

> Vec<crate::models::PermissionsMetadata200ResponseInner> permissions_metadata()
Permissions metadata

The metadata of available permissions

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PermissionsMetadata200ResponseInner>**](permissionsMetadata_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

