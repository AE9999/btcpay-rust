# WebhookDeliveryData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the delivery | [optional]
**timestamp** | Option<**f32**> | Timestamp of when the delivery got broadcasted | [optional]
**http_code** | Option<**f32**> | HTTP code received by the remote service, if any. | [optional]
**error_message** | Option<**String**> | User friendly error message, if any. | [optional]
**status** | Option<**String**> | Whether the delivery failed or not (possible values are: `Failed`, `HttpError`, `HttpSuccess`) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


