# WebhookDataBaseAuthorizedEvents

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**everything** | Option<**bool**> | If true, the endpoint will receive all events related to the store. | [optional][default to true]
**specific_events** | Option<**Vec<String>**> | If `everything` is false, the specific events that the endpoint is interested in. Current events are: `InvoiceCreated`, `InvoiceReceivedPayment`, `InvoiceProcessing`, `InvoiceExpired`, `InvoiceSettled`, `InvoiceInvalid`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


