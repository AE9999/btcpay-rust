# CreateInvoiceRequestAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amount** | Option<**String**> | The amount of the invoice. If null or unspecified, the invoice will be a top-up invoice. (ie. The invoice will consider any payment as a full payment) | [optional]
**currency** | Option<**String**> | The currency of the invoice (if null, empty or unspecified, the currency will be the store's settings default)' | [optional]
**additional_search_terms** | Option<**Vec<String>**> | Additional search term to help you find this invoice via text search | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


