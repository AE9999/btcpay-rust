# \InvoicesApi

All URIs are relative to *https://btcpay.example.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**invoices_activate_payment_method**](InvoicesApi.md#invoices_activate_payment_method) | **POST** /api/v1/stores/{storeId}/invoices/{invoiceId}/payment-methods/{paymentMethod}/activate | Activate Payment Method
[**invoices_archive_invoice**](InvoicesApi.md#invoices_archive_invoice) | **DELETE** /api/v1/stores/{storeId}/invoices/{invoiceId} | Archive invoice
[**invoices_create_invoice**](InvoicesApi.md#invoices_create_invoice) | **POST** /api/v1/stores/{storeId}/invoices | Create a new invoice
[**invoices_get_invoice**](InvoicesApi.md#invoices_get_invoice) | **GET** /api/v1/stores/{storeId}/invoices/{invoiceId} | Get invoice
[**invoices_get_invoice_payment_methods**](InvoicesApi.md#invoices_get_invoice_payment_methods) | **GET** /api/v1/stores/{storeId}/invoices/{invoiceId}/payment-methods | Get invoice payment methods
[**invoices_get_invoices**](InvoicesApi.md#invoices_get_invoices) | **GET** /api/v1/stores/{storeId}/invoices | Get invoices
[**invoices_mark_invoice_status**](InvoicesApi.md#invoices_mark_invoice_status) | **POST** /api/v1/stores/{storeId}/invoices/{invoiceId}/status | Mark invoice status
[**invoices_unarchive_invoice**](InvoicesApi.md#invoices_unarchive_invoice) | **POST** /api/v1/stores/{storeId}/invoices/{invoiceId}/unarchive | Unarchive invoice
[**invoices_update_invoice**](InvoicesApi.md#invoices_update_invoice) | **PUT** /api/v1/stores/{storeId}/invoices/{invoiceId} | Update invoice



## invoices_activate_payment_method

> invoices_activate_payment_method(store_id, invoice_id, payment_method)
Activate Payment Method

Activate an invoice payment method (if lazy payments mode is enabled)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to query | [required] |
**invoice_id** | **String** | The invoice to update | [required] |
**payment_method** | **String** | The payment method to activate | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_archive_invoice

> invoices_archive_invoice(store_id, invoice_id)
Archive invoice

Archives the specified invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store the invoice belongs to | [required] |
**invoice_id** | **String** | The invoice to remove | [required] |

### Return type

 (empty response body)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_create_invoice

> crate::models::InvoiceData invoices_create_invoice(store_id, create_invoice_request)
Create a new invoice

Create a new invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to query | [required] |
**create_invoice_request** | [**CreateInvoiceRequest**](CreateInvoiceRequest.md) |  | [required] |

### Return type

[**crate::models::InvoiceData**](InvoiceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_get_invoice

> crate::models::InvoiceData invoices_get_invoice(store_id, invoice_id)
Get invoice

View information about the specified invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**invoice_id** | **String** | The invoice to fetch | [required] |

### Return type

[**crate::models::InvoiceData**](InvoiceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_get_invoice_payment_methods

> Vec<crate::models::InvoicePaymentMethodDataModel> invoices_get_invoice_payment_methods(store_id, invoice_id, only_accounted_payments)
Get invoice payment methods

View information about the specified invoice's payment methods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to fetch | [required] |
**invoice_id** | **String** | The invoice to fetch | [required] |
**only_accounted_payments** | Option<**bool**> | If default or true, only returns payments which are accounted (in Bitcoin, this mean not returning RBF'd or double spent payments) |  |[default to true]

### Return type

[**Vec<crate::models::InvoicePaymentMethodDataModel>**](InvoicePaymentMethodDataModel.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_get_invoices

> Vec<crate::models::InvoiceData> invoices_get_invoices(store_id, order_id, status, text_search, start_date, end_date, skip, take)
Get invoices

View information about the existing invoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to query | [required] |
**order_id** | Option<[**Vec<String>**](String.md)> | Array of OrderIds to fetch the invoices for |  |
**status** | Option<[**InvoiceStatus**](.md)> | Array of statuses of invoices to be fetched |  |
**text_search** | Option<**String**> | A term that can help locating specific invoices. |  |
**start_date** | Option<**f32**> | Start date of the period to retrieve invoices |  |
**end_date** | Option<**f32**> | End date of the period to retrieve invoices |  |
**skip** | Option<**f32**> | Number of records to skip |  |
**take** | Option<**f32**> | Number of records returned in response |  |

### Return type

[**Vec<crate::models::InvoiceData>**](InvoiceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_mark_invoice_status

> crate::models::InvoiceData invoices_mark_invoice_status(store_id, invoice_id, mark_invoice_status_request)
Mark invoice status

Mark an invoice as invalid or settled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to query | [required] |
**invoice_id** | **String** | The invoice to update | [required] |
**mark_invoice_status_request** | [**MarkInvoiceStatusRequest**](MarkInvoiceStatusRequest.md) |  | [required] |

### Return type

[**crate::models::InvoiceData**](InvoiceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_unarchive_invoice

> crate::models::InvoiceData invoices_unarchive_invoice(store_id, invoice_id)
Unarchive invoice

Unarchive an invoice

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store to query | [required] |
**invoice_id** | **String** | The invoice to update | [required] |

### Return type

[**crate::models::InvoiceData**](InvoiceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## invoices_update_invoice

> crate::models::InvoiceData invoices_update_invoice(store_id, invoice_id, update_invoice_request)
Update invoice

Updates the specified invoice.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**store_id** | **String** | The store the invoice belongs to | [required] |
**invoice_id** | **String** | The invoice to update | [required] |
**update_invoice_request** | [**UpdateInvoiceRequest**](UpdateInvoiceRequest.md) |  | [required] |

### Return type

[**crate::models::InvoiceData**](InvoiceData.md)

### Authorization

[API_Key](../README.md#API_Key), [Basic](../README.md#Basic)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

