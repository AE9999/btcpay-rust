# WebhookInvoicePaymentSettledEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery_id** | Option<**String**> | The delivery id of the webhook | [optional]
**webhook_id** | Option<**String**> | The id of the webhook | [optional]
**original_delivery_id** | Option<**String**> | If this delivery is a redelivery, the is the delivery id of the original delivery. | [optional]
**is_redelivery** | Option<**bool**> | True if this delivery is a redelivery | [optional]
**_type** | Option<**String**> | The type of this event, current available are `InvoiceCreated`, `InvoiceReceivedPayment`, `InvoiceProcessing`, `InvoiceExpired`, `InvoiceSettled`, `InvoiceInvalid`, and `InvoicePaymentSettled`. | [optional]
**timestamp** | Option<**f32**> | The timestamp when this delivery has been created | [optional]
**store_id** | Option<**String**> | The store id of the invoice's event | [optional]
**invoice_id** | Option<**String**> | The invoice id of the invoice's event | [optional]
**after_expiration** | Option<**bool**> | Whether this payment has been sent after expiration of the invoice | [optional]
**payment_method** | Option<**String**> | What payment method was used for this payment | [optional]
**payment** | Option<[**crate::models::Payment**](Payment.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


