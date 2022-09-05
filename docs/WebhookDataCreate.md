# WebhookDataCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The id of the webhook | [optional]
**enabled** | Option<**bool**> | Whether this webhook is enabled or not | [optional][default to true]
**automatic_redelivery** | Option<**bool**> | If true, BTCPay Server will retry to redeliver any failed delivery after 10 seconds, 1 minutes and up to 6 times after 10 minutes. | [optional][default to true]
**url** | Option<**String**> | The endpoint where BTCPay Server will make the POST request with the webhook body | [optional]
**authorized_events** | Option<[**crate::models::WebhookDataBaseAuthorizedEvents**](WebhookDataBase_authorizedEvents.md)> |  | [optional]
**secret** | Option<**String**> | Must be used by the callback receiver to ensure the delivery comes from BTCPay Server. BTCPay Server includes the `BTCPay-Sig` HTTP header, whose format is `sha256=HMAC256(UTF8(webhook's secret), body)`. The pattern to authenticate the webhook is similar to [how to secure webhooks in Github](https://docs.github.com/webhooks/securing/). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


