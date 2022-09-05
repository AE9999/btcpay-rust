# InvoiceDataAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The identifier of the invoice | [optional]
**store_id** | Option<**String**> | The store identifier that the invoice belongs to | [optional]
**amount** | Option<**String**> | The amount of the invoice | [optional]
**currency** | Option<**String**> | The currency of the invoice | [optional]
**_type** | Option<[**crate::models::InvoiceType**](InvoiceType.md)> |  | [optional]
**checkout_link** | Option<**String**> | The link to the checkout page, where you can redirect the customer | [optional]
**created_time** | Option<**f32**> | The creation time of the invoice | [optional]
**expiration_time** | Option<**f32**> | The expiration time of the invoice | [optional]
**monitoring_time** | Option<**f32**> | The monitoring time of the invoice | [optional]
**status** | Option<[**crate::models::InvoiceStatus**](InvoiceStatus.md)> |  | [optional]
**additional_status** | Option<[**crate::models::InvoiceAdditionalStatus**](InvoiceAdditionalStatus.md)> |  | [optional]
**available_statuses_for_manual_marking** | Option<[**Vec<crate::models::InvoiceStatus>**](InvoiceStatus.md)> | The statuses the invoice can be manually marked as | [optional]
**archived** | Option<**bool**> | true if the invoice is archived | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


